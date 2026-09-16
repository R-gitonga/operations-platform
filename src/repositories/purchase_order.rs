use sqlx::{query_as, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::purchase_order::PurchaseOrder,
};

// supplier_name is resolved via a scalar subquery rather than a
// JOIN so the exact same expression can be reused verbatim in
// every RETURNING clause below (Postgres allows a RETURNING
// clause to reference other tables via subquery, but not a JOIN
// against the statement's own target table). Wrapped in
// COALESCE-free form since supplier_id is FK-RESTRICTed — a
// matching supplier row is always guaranteed to exist.
const SUPPLIER_NAME_SUBQUERY: &str =
    "(SELECT name FROM suppliers WHERE id = purchase_orders.supplier_id)";

const PURCHASE_ORDER_SELECT_COLUMNS: &str = r#"
    id,
    erp_reference,
    accounts_reference,
    supplier_id,
    __SUPPLIER_NAME__ AS supplier_name,
    description,
    attachment_path,
    attachment_name,
    status,
    created_by,
    created_at,
    updated_at
"#;

fn select_columns() -> String {
    PURCHASE_ORDER_SELECT_COLUMNS.replace("__SUPPLIER_NAME__", SUPPLIER_NAME_SUBQUERY)
}

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    erp_reference: &str,
    accounts_reference: &str,
    supplier_id: i32,
    description: Option<&str>,
    created_by: &str,
) -> Result<PurchaseOrder, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            INSERT INTO purchase_orders
            (
                erp_reference,
                accounts_reference,
                supplier_id,
                description,
                status,
                created_by
            )
            VALUES
            (
                $1, $2, $3, $4, 'active', $5
            )
            RETURNING {}
            "#,
            select_columns()
        ),
    )
    .bind(erp_reference)
    .bind(accounts_reference)
    .bind(supplier_id)
    .bind(description)
    .bind(created_by)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<PurchaseOrder>, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            "SELECT {} FROM purchase_orders ORDER BY id DESC",
            select_columns()
        ),
    )
    .fetch_all(pool)
    .await
}

pub async fn find_all_filtered(
    pool: &DbPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<PurchaseOrder>, sqlx::Error> {

    let search_pattern =
        search.map(|v| format!("%{}%", v));

    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            SELECT {}
            FROM purchase_orders

            WHERE

                ($1::TEXT IS NULL
                    OR erp_reference ILIKE $1
                    OR accounts_reference ILIKE $1)

            AND

                ($2::TEXT IS NULL OR status = $2)

            ORDER BY id DESC
            "#,
            select_columns()
        ),
    )
    .bind(search_pattern)
    .bind(status)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<PurchaseOrder>, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            "SELECT {} FROM purchase_orders WHERE id = $1",
            select_columns()
        ),
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
) -> Result<Option<PurchaseOrder>, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            "SELECT {} FROM purchase_orders WHERE id = $1",
            select_columns()
        ),
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    erp_reference: &str,
    accounts_reference: &str,
    supplier_id: i32,
    description: Option<&str>,
) -> Result<PurchaseOrder, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            UPDATE purchase_orders
            SET
                erp_reference = $2,
                accounts_reference = $3,
                supplier_id = $4,
                description = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING {}
            "#,
            select_columns()
        ),
    )
    .bind(id)
    .bind(erp_reference)
    .bind(accounts_reference)
    .bind(supplier_id)
    .bind(description)
    .fetch_one(pool)
    .await
}

pub async fn update_attachment(
    pool: &DbPool,
    id: i32,
    attachment_name: &str,
    attachment_path: &str,
) -> Result<PurchaseOrder, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            UPDATE purchase_orders
            SET
                attachment_name = $2,
                attachment_path = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING {}
            "#,
            select_columns()
        ),
    )
    .bind(id)
    .bind(attachment_name)
    .bind(attachment_path)
    .fetch_one(pool)
    .await
}

pub async fn cancel(
    pool: &DbPool,
    id: i32,
) -> Result<PurchaseOrder, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            UPDATE purchase_orders
            SET
                status = 'cancelled',
                updated_at = NOW()
            WHERE id = $1
            RETURNING {}
            "#,
            select_columns()
        ),
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn reactivate(
    pool: &DbPool,
    id: i32,
) -> Result<PurchaseOrder, sqlx::Error> {
    query_as::<_, PurchaseOrder>(
        &format!(
            r#"
            UPDATE purchase_orders
            SET
                status = 'active',
                updated_at = NOW()
            WHERE id = $1
            RETURNING {}
            "#,
            select_columns()
        ),
    )
    .bind(id)
    .fetch_one(pool)
    .await
}