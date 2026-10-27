use crate::{database::DbPool, errors::app_error::AppError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Postgres, Transaction};

pub fn tables(kind: &str) -> Result<(&'static str, &'static str, &'static str), AppError> {
    match kind {
        "wso" => Ok(("wso_orders", "wso_items", "wso_order_id")),
        "po" => Ok(("purchase_orders", "po_items", "purchase_order_id")),
        _ => Err(AppError::BadRequest("Unknown order type".into())),
    }
}
pub fn validate_reason(paid: bool, reason: Option<&str>) -> Result<(), AppError> {
    if (paid && !matches!(reason, Some("mtm" | "stock_out"))) || (!paid && reason.is_some()) {
        return Err(AppError::BadRequest(
            "Each paid item needs an MTM or Stock-out reason; bulk items must have no reason."
                .into(),
        ));
    }
    Ok(())
}
pub async fn actor(tx: &mut Transaction<'_, Postgres>, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('app.actor',$1,true)")
        .bind(name)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
pub async fn set_header(
    tx: &mut Transaction<'_, Postgres>,
    kind: &str,
    id: i32,
    paid: bool,
    branch: Option<i32>,
) -> Result<(), AppError> {
    let (table, _, _) = tables(kind)?;
    if paid != branch.is_some() {
        return Err(AppError::BadRequest(
            "Paid orders require a branch; bulk orders cannot have a branch.".into(),
        ));
    }
    let current: Option<i32> = sqlx::query_scalar(&format!(
        "SELECT branch_id FROM {table} WHERE id=$1 FOR UPDATE"
    ))
    .bind(id)
    .fetch_one(&mut **tx)
    .await?;
    if branch.is_some() && branch != current {
        let active: Option<bool> =
            sqlx::query_scalar("SELECT active FROM branches WHERE id=$1 FOR SHARE")
                .bind(branch)
                .fetch_optional(&mut **tx)
                .await?;
        if active != Some(true) {
            return Err(AppError::BadRequest("Select an active branch.".into()));
        }
    }
    sqlx::query(&format!(
        "UPDATE {table} SET is_paid_order=$2,branch_id=$3 WHERE id=$1"
    ))
    .bind(id)
    .bind(paid)
    .bind(branch)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[derive(Deserialize)]
pub struct ItemReason {
    pub id: i32,
    pub paid_order_reason: Option<String>,
}
#[derive(Deserialize)]
pub struct Classification {
    pub is_paid_order: bool,
    pub branch_id: Option<i32>,
    pub items: Vec<ItemReason>,
}

pub async fn metadata(pool: &DbPool, kind: &str, id: i32) -> Result<Value, AppError> {
    let (table, items, fk) = tables(kind)?;
    Ok(sqlx::query_scalar(&format!("SELECT jsonb_build_object('is_paid_order',o.is_paid_order,'branch_id',o.branch_id,'branch_name',b.name,'items',coalesce((SELECT jsonb_agg(jsonb_build_object('id',i.id,'description',i.description,'paid_order_reason',i.paid_order_reason) ORDER BY i.id) FROM {items} i WHERE i.{fk}=o.id),'[]'::jsonb)) FROM {table} o LEFT JOIN branches b ON b.id=o.branch_id WHERE o.id=$1"))
        .bind(id).fetch_one(pool).await?)
}

pub async fn correct(
    pool: &DbPool,
    kind: &str,
    id: i32,
    request: &Classification,
    name: &str,
) -> Result<Value, AppError> {
    let (table, items, fk) = tables(kind)?;
    let mut tx = pool.begin().await?;
    actor(&mut tx, name).await?;
    let status: String = sqlx::query_scalar(&format!(
        "SELECT status FROM {table} WHERE id=$1 FOR UPDATE"
    ))
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;
    if status == "cancelled" {
        return Err(AppError::BadRequest(
            "Reactivate the order before editing classification.".into(),
        ));
    }
    let ids: Vec<i32> = sqlx::query_scalar(&format!(
        "SELECT id FROM {items} WHERE {fk}=$1 ORDER BY id FOR UPDATE"
    ))
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    let mut supplied: Vec<i32> = request.items.iter().map(|i| i.id).collect();
    supplied.sort();
    if ids != supplied {
        return Err(AppError::BadRequest(
            "The item list changed. Refresh and provide every item exactly once.".into(),
        ));
    }
    for item in &request.items {
        validate_reason(request.is_paid_order, item.paid_order_reason.as_deref())?;
    }
    set_header(&mut tx, kind, id, request.is_paid_order, request.branch_id).await?;
    for item in &request.items {
        sqlx::query(&format!(
            "UPDATE {items} SET paid_order_reason=$2 WHERE id=$1"
        ))
        .bind(item.id)
        .bind(&item.paid_order_reason)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    metadata(pool, kind, id).await
}

#[derive(Deserialize, Default)]
pub struct Filters {
    pub branch_id: Option<i32>,
    pub reason: Option<String>,
    pub category_id: Option<i32>,
    pub order_type: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<chrono::NaiveDate>,
    pub date_to: Option<chrono::NaiveDate>,
    pub page: Option<i64>,
}
#[derive(Serialize)]
pub struct Page {
    pub items: Vec<Value>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
pub async fn list(pool: &DbPool, f: &Filters) -> Result<Page, AppError> {
    if let Some(reason) = &f.reason {
        validate_reason(true, Some(reason))?;
    }
    if let Some(kind) = &f.order_type {
        tables(kind)?;
    }
    let status = f.status.as_deref().unwrap_or("open");
    if !["all", "open", "active", "partial", "completed", "cancelled"].contains(&status) {
        return Err(AppError::BadRequest("Invalid status filter".into()));
    }
    if f.date_from.zip(f.date_to).is_some_and(|(a, b)| a > b) {
        return Err(AppError::BadRequest(
            "Start date must precede end date".into(),
        ));
    }
    let page = f.page.unwrap_or(1).clamp(1, 1_000_000);
    // Filter item dimensions together, then group once per parent. Quantities
    // deliberately describe matching items, never an unfiltered order total.
    let sql = r#"WITH grouped AS (
      SELECT i.order_type,i.order_id,i.reference,i.secondary_reference,i.branch_id,b.name branch_name,
        i.created_at,i.status,array_agg(DISTINCT i.paid_order_reason) reasons,
        array_agg(DISTINCT i.category_name) FILTER(WHERE i.category_name IS NOT NULL) categories,
        sum(i.ordered)::bigint ordered,sum(i.fulfilled)::bigint fulfilled,
        sum(greatest(i.ordered-i.fulfilled,0))::bigint outstanding,
        jsonb_agg(jsonb_build_object('id',i.item_id,'description',i.description,'reason',i.paid_order_reason,'category',i.category_name) ORDER BY i.item_id) matched_items
      FROM paid_order_items i JOIN branches b ON b.id=i.branch_id
      WHERE ($1::int IS NULL OR i.branch_id=$1) AND ($2::text IS NULL OR i.paid_order_reason=$2)
        AND ($3::int IS NULL OR i.category_id=$3) AND ($4::text IS NULL OR i.order_type=$4)
        AND ($5='all' OR ($5='open' AND i.status NOT IN ('completed','cancelled')) OR i.status=$5)
        AND ($6::text IS NULL OR i.reference ILIKE $6 OR i.secondary_reference ILIKE $6)
        AND ($7::date IS NULL OR i.created_at >= ($7::timestamp AT TIME ZONE 'Africa/Nairobi'))
        AND ($8::date IS NULL OR i.created_at < (($8+1)::timestamp AT TIME ZONE 'Africa/Nairobi'))
      GROUP BY i.order_type,i.order_id,i.reference,i.secondary_reference,i.branch_id,b.name,i.created_at,i.status
    ), selected AS (SELECT * FROM grouped ORDER BY created_at,order_type,order_id LIMIT 25 OFFSET $9)
    SELECT jsonb_build_object('total',(SELECT count(*) FROM grouped),'items',coalesce((SELECT jsonb_agg(to_jsonb(s) ORDER BY created_at,order_type,order_id) FROM selected s),'[]'::jsonb))"#;
    let result: Value = sqlx::query_scalar(sql)
        .bind(f.branch_id)
        .bind(&f.reason)
        .bind(f.category_id)
        .bind(&f.order_type)
        .bind(status)
        .bind(f.search.as_ref().map(|s| format!("%{}%", s.trim())))
        .bind(f.date_from)
        .bind(f.date_to)
        .bind((page - 1) * 25)
        .fetch_one(pool)
        .await?;
    Ok(Page {
        items: result["items"].as_array().cloned().unwrap_or_default(),
        total: result["total"].as_i64().unwrap_or(0),
        page,
        page_size: 25,
    })
}
