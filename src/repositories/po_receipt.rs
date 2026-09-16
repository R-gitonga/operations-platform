use sqlx::{query_as, query_scalar, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::{
        po_receipt::{CreatePoReceiptRequest, PoReceipt, PoReceiptDetail},
        po_receipt_line::PoReceiptLine,
    },
};

const PO_RECEIPT_SELECT: &str = r#"
SELECT
    id,
    purchase_order_id,
    delivery_note_reference,
    notes,
    received_by,
    received_at,
    created_at
FROM po_receipts
"#;

const PO_RECEIPT_LINE_SELECT: &str = r#"
SELECT
    id,
    po_receipt_id,
    po_line_item_id,
    qty_delivered,
    total_delivered_to_date,
    delivered_balance
FROM po_receipt_lines
"#;

async fn total_delivered_for_line_tx(
    tx: &mut Transaction<'_, Postgres>,
    po_line_item_id: i32,
) -> Result<i32, sqlx::Error> {
    let total: i64 = query_scalar(
        "SELECT COALESCE(SUM(qty_delivered), 0) FROM po_receipt_lines WHERE po_line_item_id = $1",
    )
    .bind(po_line_item_id)
    .fetch_one(tx.as_mut())
    .await?;

    Ok(total as i32)
}

pub async fn create(
    pool: &DbPool,
    purchase_order_id: i32,
    received_by: &str,
    payload: &CreatePoReceiptRequest,
) -> Result<PoReceiptDetail, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let receipt = query_as::<_, PoReceipt>(
        r#"
        INSERT INTO po_receipts
        (
            purchase_order_id,
            delivery_note_reference,
            notes,
            received_by
        )
        VALUES
        (
            $1, $2, $3, $4
        )
        RETURNING
            id,
            purchase_order_id,
            delivery_note_reference,
            notes,
            received_by,
            received_at,
            created_at
        "#,
    )
    .bind(purchase_order_id)
    .bind(&payload.delivery_note_reference)
    .bind(&payload.notes)
    .bind(received_by)
    .fetch_one(tx.as_mut())
    .await?;

    let mut lines = Vec::with_capacity(payload.lines.len());

    for line in &payload.lines {
        let qty_ordered: i32 = query_scalar(
            "SELECT qty_ordered FROM po_line_items WHERE id = $1",
        )
        .bind(line.po_line_item_id)
        .fetch_one(tx.as_mut())
        .await?;

        let previously_delivered =
            total_delivered_for_line_tx(&mut tx, line.po_line_item_id).await?;

        let total_delivered_to_date = previously_delivered + line.qty_delivered;
        let delivered_balance = qty_ordered - total_delivered_to_date;

        let receipt_line = query_as::<_, PoReceiptLine>(
            r#"
            INSERT INTO po_receipt_lines
            (
                po_receipt_id,
                po_line_item_id,
                qty_delivered,
                total_delivered_to_date,
                delivered_balance
            )
            VALUES
            (
                $1, $2, $3, $4, $5
            )
            RETURNING
                id,
                po_receipt_id,
                po_line_item_id,
                qty_delivered,
                total_delivered_to_date,
                delivered_balance
            "#,
        )
        .bind(receipt.id)
        .bind(line.po_line_item_id)
        .bind(line.qty_delivered)
        .bind(total_delivered_to_date)
        .bind(delivered_balance)
        .fetch_one(tx.as_mut())
        .await?;

        lines.push(receipt_line);
    }

    tx.commit().await?;

    Ok(PoReceiptDetail { receipt, lines })
}

pub async fn find_by_purchase_order(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<Vec<PoReceipt>, sqlx::Error> {
    query_as::<_, PoReceipt>(
        &format!("{PO_RECEIPT_SELECT} WHERE purchase_order_id = $1 ORDER BY received_at DESC"),
    )
    .bind(purchase_order_id)
    .fetch_all(pool)
    .await
}

pub async fn find_lines_by_receipt(
    pool: &DbPool,
    po_receipt_id: i32,
) -> Result<Vec<PoReceiptLine>, sqlx::Error> {
    query_as::<_, PoReceiptLine>(
        &format!("{PO_RECEIPT_LINE_SELECT} WHERE po_receipt_id = $1 ORDER BY id"),
    )
    .bind(po_receipt_id)
    .fetch_all(pool)
    .await
}

pub async fn find_lines_by_line_item(
    pool: &DbPool,
    po_line_item_id: i32,
) -> Result<Vec<PoReceiptLine>, sqlx::Error> {
    query_as::<_, PoReceiptLine>(
        &format!("{PO_RECEIPT_LINE_SELECT} WHERE po_line_item_id = $1 ORDER BY id"),
    )
    .bind(po_line_item_id)
    .fetch_all(pool)
    .await
}

pub async fn total_delivered_for_line(
    pool: &DbPool,
    po_line_item_id: i32,
) -> Result<i32, sqlx::Error> {
    let total: i64 = query_scalar(
        "SELECT COALESCE(SUM(qty_delivered), 0) FROM po_receipt_lines WHERE po_line_item_id = $1",
    )
    .bind(po_line_item_id)
    .fetch_one(pool)
    .await?;

    Ok(total as i32)
}