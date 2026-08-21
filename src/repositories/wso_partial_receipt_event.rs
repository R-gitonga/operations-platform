use sqlx::{query_as, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::wso_partial_receipt_event::WsoPartialReceiptEvent,
};

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    line_item_id: i32,
    quantity_received: i32,
    total_raised: i32,
    total_received: i32,
    balance: i32,
    received_by: &str,
) -> Result<WsoPartialReceiptEvent, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let event = create_tx(
        &mut tx,
        wso_item_id,
        line_item_id,
        quantity_received,
        total_raised,
        total_received,
        balance,
        received_by,
    )
    .await?;

    tx.commit().await?;

    Ok(event)
}

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    wso_item_id: i32,
    line_item_id: i32,
    quantity_received: i32,
    total_raised: i32,
    total_received: i32,
    balance: i32,
    received_by: &str,
) -> Result<WsoPartialReceiptEvent, sqlx::Error> {
    query_as::<_, WsoPartialReceiptEvent>(
        r#"
        INSERT INTO wso_partial_receipt_events
        (
            wso_item_id,
            line_item_id,
            quantity_received,
            total_raised,
            total_received,
            balance,
            received_by
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7
        )
        RETURNING
            id,
            wso_item_id,
            line_item_id,
            quantity_received,
            total_raised,
            total_received,
            balance,
            received_by,
            received_at
        "#,
    )
    .bind(wso_item_id)
    .bind(line_item_id)
    .bind(quantity_received)
    .bind(total_raised)
    .bind(total_received)
    .bind(balance)
    .bind(received_by)
    .fetch_one(tx.as_mut())
    .await
}