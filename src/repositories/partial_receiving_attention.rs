use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::partial_receiving_attention_item::PartialReceivingAttentionItem,
    repositories::partial_receiving_tracking,
};

pub async fn find_attention_required(
    pool: &DbPool,
) -> Result<Vec<PartialReceivingAttentionItem>, sqlx::Error> {
    sqlx::query_as::<_, PartialReceivingAttentionItem>(
        r#"
        SELECT
            prt.id AS tracking_id,

            wo.id AS wso_id,

            wi.id AS wso_item_id,

            wo.wso_number,

            wi.description,

            wi.design_code,

            wi.fabric_code,

            prt.first_partial_received_at,

            prs.attention_after_days,

            FLOOR(
                EXTRACT(
                    EPOCH FROM (
                        CURRENT_TIMESTAMP
                        - prt.first_partial_received_at
                    )
                ) / 86400
            )::BIGINT AS elapsed_days,

            FLOOR(
                EXTRACT(
                    EPOCH FROM (
                        CURRENT_TIMESTAMP
                        - prt.first_partial_received_at
                    )
                ) / 86400
            )::BIGINT
            - prs.attention_after_days
            AS overdue_days,

            COALESCE(
                (
                    SELECT SUM(
                        wli.qty_raised - wli.qty_received
                    )
                    FROM wso_line_items wli
                    WHERE wli.wso_item_id = wi.id
                ),
                0
            ) AS outstanding_quantity

        FROM partial_receiving_tracking prt

        JOIN wso_items wi
            ON wi.id = prt.wso_item_id

        JOIN wso_orders wo
            ON wo.id = wi.wso_order_id

        CROSS JOIN partial_receiving_settings prs

        WHERE prt.resolved_at IS NULL

          AND LOWER(wo.status) = 'active'

          AND prs.attention_after_days <=
              FLOOR(
                  EXTRACT(
                      EPOCH FROM (
                          CURRENT_TIMESTAMP
                          - prt.first_partial_received_at
                      )
                  ) / 86400
              )::INTEGER

          AND EXISTS (
              SELECT 1
              FROM wso_line_items wli
              WHERE wli.wso_item_id = wi.id
                AND wli.qty_received > 0
          )

          AND EXISTS (
              SELECT 1
              FROM wso_line_items wli
              WHERE wli.wso_item_id = wi.id
                AND wli.qty_raised > wli.qty_received
          )

        ORDER BY
            overdue_days DESC,
            prt.first_partial_received_at ASC,
            wo.wso_number,
            wi.description
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_attention_required_items(
    pool: &DbPool,
) -> Result<Vec<PartialReceivingAttentionItem>, AppError> {
    Ok(
        partial_receiving_tracking::find_attention_required(
            pool
        )
        .await?
    )
}