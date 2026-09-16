use sqlx::{query_as, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::po_line_item::{CreatePoLineItemRequest, PoLineItem},
};

const PO_LINE_ITEM_SELECT: &str = r#"
SELECT
    id,
    po_item_id,
    size,
    qty_ordered,
    created_at,
    updated_at
FROM po_line_items
"#;

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    po_item_id: i32,
    payload: &CreatePoLineItemRequest,
) -> Result<PoLineItem, sqlx::Error> {
    query_as::<_, PoLineItem>(
        r#"
        INSERT INTO po_line_items
        (
            po_item_id,
            size,
            qty_ordered
        )
        VALUES
        (
            $1, $2, $3
        )
        RETURNING
            id,
            po_item_id,
            size,
            qty_ordered,
            created_at,
            updated_at
        "#,
    )
    .bind(po_item_id)
    .bind(&payload.size)
    .bind(payload.qty_ordered)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn create(
    pool: &DbPool,
    po_item_id: i32,
    payload: &CreatePoLineItemRequest,
) -> Result<PoLineItem, sqlx::Error> {
    query_as::<_, PoLineItem>(
        r#"
        INSERT INTO po_line_items
        (
            po_item_id,
            size,
            qty_ordered
        )
        VALUES
        (
            $1, $2, $3
        )
        RETURNING
            id,
            po_item_id,
            size,
            qty_ordered,
            created_at,
            updated_at
        "#,
    )
    .bind(po_item_id)
    .bind(&payload.size)
    .bind(payload.qty_ordered)
    .fetch_one(pool)
    .await
}

pub async fn find_by_item(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoLineItem>, sqlx::Error> {
    query_as::<_, PoLineItem>(
        &format!("{} WHERE po_item_id = $1 ORDER BY id", PO_LINE_ITEM_SELECT),
    )
    .bind(po_item_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<PoLineItem>, sqlx::Error> {
    query_as::<_, PoLineItem>(
        &format!("{} WHERE id = $1", PO_LINE_ITEM_SELECT),
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    size: &str,
    qty_ordered: i32,
) -> Result<PoLineItem, sqlx::Error> {
    query_as::<_, PoLineItem>(
        r#"
        UPDATE po_line_items
        SET
            size = $2,
            qty_ordered = $3,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            po_item_id,
            size,
            qty_ordered,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .bind(size)
    .bind(qty_ordered)
    .fetch_one(pool)
    .await
}