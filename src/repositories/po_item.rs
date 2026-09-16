use sqlx::{query_as, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::po_item::{CreatePoItemRequest, PoItem},
};

const PO_ITEM_SELECT: &str = r#"
SELECT
    pi.id,
    pi.purchase_order_id,
    pi.category_id,
    pi.description,
    pi.expected_delivery_date,
    pi.branding_required,
    pi.branding_type_id,
    bt.display_name AS branding_type_name,
    pi.branding_location_id,
    bl.display_name AS branding_location_name,
    pi.created_by,
    pi.created_at,
    pi.updated_at

FROM po_items pi

LEFT JOIN branding_types bt
    ON bt.id = pi.branding_type_id

LEFT JOIN branding_locations bl
    ON bl.id = pi.branding_location_id
"#;

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    purchase_order_id: i32,
    payload: &CreatePoItemRequest,
    created_by: &str,
) -> Result<PoItem, sqlx::Error> {
    let id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO po_items
        (
            purchase_order_id,
            category_id,
            description,
            expected_delivery_date,
            branding_required,
            branding_type_id,
            branding_location_id,
            created_by
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8
        )
        RETURNING id
        "#,
    )
    .bind(purchase_order_id)
    .bind(payload.category_id)
    .bind(&payload.description)
    .bind(payload.expected_delivery_date)
    .bind(payload.branding_required)
    .bind(payload.branding_type_id)
    .bind(payload.branding_location_id)
    .bind(created_by)
    .fetch_one(tx.as_mut())
    .await?;

    query_as::<_, PoItem>(
        &format!("{} WHERE pi.id = $1", PO_ITEM_SELECT),
    )
    .bind(id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn find_by_purchase_order(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<Vec<PoItem>, sqlx::Error> {
    query_as::<_, PoItem>(
        &format!(
            "{} WHERE pi.purchase_order_id = $1 ORDER BY pi.id",
            PO_ITEM_SELECT
        ),
    )
    .bind(purchase_order_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<PoItem>, sqlx::Error> {
    query_as::<_, PoItem>(
        &format!("{} WHERE pi.id = $1", PO_ITEM_SELECT),
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    category_id: Option<i32>,
    description: Option<&str>,
    expected_delivery_date: Option<chrono::NaiveDate>,
    branding_required: bool,
    branding_type_id: Option<i32>,
    branding_location_id: Option<i32>,
) -> Result<PoItem, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE po_items
        SET
            category_id = $2,
            description = $3,
            expected_delivery_date = $4,
            branding_required = $5,
            branding_type_id = $6,
            branding_location_id = $7,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(category_id)
    .bind(description)
    .bind(expected_delivery_date)
    .bind(branding_required)
    .bind(branding_type_id)
    .bind(branding_location_id)
    .execute(pool)
    .await?;

    query_as::<_, PoItem>(
        &format!("{} WHERE pi.id = $1", PO_ITEM_SELECT),
    )
    .bind(id)
    .fetch_one(pool)
    .await
}