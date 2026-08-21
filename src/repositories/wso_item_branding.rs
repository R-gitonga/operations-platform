use crate::{
    database::DbPool,
    models::{
        wso_item_branding::WsoItemBranding,
        wso_item_branding_detail::WsoItemBrandingDetail,
    },
};

use sqlx::{Postgres, Transaction};

pub async fn find_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoItemBranding>, sqlx::Error> {
    sqlx::query_as::<_, WsoItemBranding>(
        r#"
        SELECT
            id,
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity,
            created_at,
            updated_at
        FROM wso_item_branding
        WHERE wso_item_id = $1
        ORDER BY id
        "#,
    )
    .bind(wso_item_id)
    .fetch_all(pool)
    .await
}

pub async fn find_details_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoItemBrandingDetail>, sqlx::Error> {
    sqlx::query_as::<_, WsoItemBrandingDetail>(
        r#"
        SELECT
            wib.id,
            wib.wso_item_id,

            bt.id AS branding_type_id,
            bt.code AS branding_type_code,
            bt.display_name AS branding_type_name,

            bl.id AS branding_location_id,
            bl.code AS branding_location_code,
            bl.display_name AS branding_location_name,

            wib.quantity,

            wib.created_at,
            wib.updated_at

        FROM wso_item_branding wib

        JOIN branding_types bt
            ON bt.id = wib.branding_type_id

        JOIN branding_locations bl
            ON bl.id = wib.branding_location_id

        WHERE wib.wso_item_id = $1

        ORDER BY
            bt.display_order,
            bl.display_order,
            wib.id
        "#,
    )
    .bind(wso_item_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<WsoItemBranding>, sqlx::Error> {
    sqlx::query_as::<_, WsoItemBranding>(
        r#"
        SELECT
            id,
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity,
            created_at,
            updated_at
        FROM wso_item_branding
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    branding_type_id: i32,
    branding_location_id: i32,
    quantity: i32,
) -> Result<WsoItemBranding, sqlx::Error> {
    sqlx::query_as::<_, WsoItemBranding>(
        r#"
        INSERT INTO wso_item_branding (
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity,
            created_at,
            updated_at
        "#,
    )
    .bind(wso_item_id)
    .bind(branding_type_id)
    .bind(branding_location_id)
    .bind(quantity)
    .fetch_one(pool)
    .await
}

pub async fn delete(
    pool: &DbPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM wso_item_branding
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}




pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    wso_item_id: i32,
    branding_type_id: i32,
    branding_location_id: i32,
    quantity: i32,
) -> Result<WsoItemBranding, sqlx::Error> {
    sqlx::query_as::<_, WsoItemBranding>(
        r#"
        INSERT INTO wso_item_branding (
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity,
            created_at,
            updated_at
        "#,
    )
    .bind(wso_item_id)
    .bind(branding_type_id)
    .bind(branding_location_id)
    .bind(quantity)
    .fetch_one(tx.as_mut())
    .await
}