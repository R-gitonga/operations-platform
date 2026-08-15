use chrono::{DateTime, Utc};
use sqlx::query_as;

use crate::{
    database::DbPool,
    models::{
        partial_receiving_attention_item::PartialReceivingAttentionItem,
        partial_receiving_tracking::PartialReceivingTracking,
    },
};

pub async fn find_active_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Option<PartialReceivingTracking>, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        SELECT
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        FROM partial_receiving_tracking
        WHERE wso_item_id = $1
          AND resolved_at IS NULL
        ORDER BY first_partial_received_at DESC
        LIMIT 1
        "#,
    )
    .bind(wso_item_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    first_partial_received_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        INSERT INTO partial_receiving_tracking (
            wso_item_id,
            first_partial_received_at
        )
        VALUES ($1, $2)
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(wso_item_id)
    .bind(first_partial_received_at)
    .fetch_one(pool)
    .await
}

pub async fn mark_notification_sent(
    pool: &DbPool,
    id: i32,
    sent_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        UPDATE partial_receiving_tracking
        SET
            notification_sent_at = $1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(sent_at)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn resolve(
    pool: &DbPool,
    id: i32,
    resolved_at: DateTime<Utc>,
) -> Result<PartialReceivingTracking, sqlx::Error> {
    query_as::<_, PartialReceivingTracking>(
        r#"
        UPDATE partial_receiving_tracking
        SET
            resolved_at = $1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING
            id,
            wso_item_id,
            first_partial_received_at,
            notification_sent_at,
            resolved_at,
            created_at,
            updated_at
        "#,
    )
    .bind(resolved_at)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_attention_required(
    pool: &DbPool,
) -> Result<Vec<PartialReceivingAttentionItem>, sqlx::Error> {
    query_as::<_, PartialReceivingAttentionItem>(
        r#"
        SELECT
            prt.id AS tracking_id,

            wo.id AS wso_id,
            wo.wso_number,

            wi.id AS wso_item_id,
            wi.description,
            wi.design_code,
            wi.fabric_code,

            prt.first_partial_received_at,

            totals.total_qty_raised,
            totals.total_qty_received,

            totals.total_balance AS balance,

            prs.attention_after_days

        FROM partial_receiving_tracking prt

        JOIN wso_items wi
            ON wi.id = prt.wso_item_id

        JOIN wso_orders wo
            ON wo.id = wi.wso_order_id

        CROSS JOIN partial_receiving_settings prs

        JOIN LATERAL (
            SELECT
                COALESCE(SUM(li.qty_raised), 0)::INTEGER
                    AS total_qty_raised,

                COALESCE(SUM(li.qty_received), 0)::INTEGER
                    AS total_qty_received,

                COALESCE(SUM(li.qty_raised - li.qty_received), 0)::INTEGER
                    AS total_balance

            FROM wso_line_items li

            WHERE li.wso_item_id = wi.id
        ) totals ON TRUE

        WHERE prt.resolved_at IS NULL

          AND totals.total_qty_received  > 0

          AND totals.total_balance > 0

          AND prt.first_partial_received_at
              <= NOW()
                 - make_interval(
                     days => prs.attention_after_days
                   )

          AND prt.notification_sent_at IS NULL

        ORDER BY
            prt.first_partial_received_at ASC,
            wo.wso_number,
            wi.description
        "#,
    )
    .fetch_all(pool)
    .await
}