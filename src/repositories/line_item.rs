use sqlx::query_as;

use crate::{
    database::DbPool,
    models::line_item::{
        CreateWsoLineItemRequest,
        WsoLineItem,
    },
};

pub async fn create(
    pool: &DbPool,
    wso_order_id: i32,
    payload: &CreateWsoLineItemRequest,
) -> Result<WsoLineItem, sqlx::Error> {

    query_as::<_, WsoLineItem>(
    r#"
        INSERT INTO wso_line_items (
            wso_order_id,
            size,
            quantity
            )
        VALUES (
            $1, $2, $3
            )
        RETURNING
            ID,
            wso_order_id,
            size,
            quantity
        "#,
    )
    .bind(wso_order_id)
    .bind(&payload.size)
    .bind(payload.quantity)
    .fetch_one(pool)
    .await
}

pub async fn find_by_wso(
    pool: &DbPool,
    wso_order_id: i32,
) -> Result<Vec<WsoLineItem>, sqlx::Error> {

    query_as::<_, WsoLineItem>(
        r#"
        SELECT
            id,
            wso_order_id,
            size,
            quantity
        FROM 
            wso_line_items
        WHERE wso_order_id = $1
        ORDER BY id ASC
        "#,
    )
    .bind(wso_order_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    line_item_id: i32,
) -> Result<WsoLineItem, sqlx::Error> {
    query_as::<_, WsoLineItem>(
        r#"
        SELECT
            id,
            wso_order_id,
            size,
            quantity
        FROM wso_line_items
        WHERE id = $1
        "#,
    )
    .bind(line_item_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    item: &WsoLineItem,
) -> Result<WsoLineItem, sqlx::Error> {
    query_as::<_, WsoLineItem>(
        r#"
        UPDATE wso_line_items
        SET
            size = $1,
            quantity = $2
        WHERE id = $3
        RETURNING
            id,
            wso_order_id,
            size,
            quantity
        "#,
    )
    .bind(&item.size)
    .bind(item.quantity)
    .bind(item.id)
    .fetch_one(pool)
    .await
}

pub async fn delete(
    pool: &DbPool,
    line_item_id: i32,
) -> Result<WsoLineItem, sqlx::Error> {
    query_as::<_, WsoLineItem>(
        r#"
        DELETE FROM wso_line_items
        WHERE id = $1
        RETURNING
            id,
            wso_order_id,
            size,
            quantity
        "#,
    )
    .bind(line_item_id)
    .fetch_one(pool)
    .await
}