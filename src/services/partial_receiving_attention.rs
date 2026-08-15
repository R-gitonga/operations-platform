use chrono::Utc;
use std::collections::HashMap;

use crate::{
    config::Config,
    database::DbPool,
    errors::app_error::AppError,
    models::{
        notification_context::NotificationContext,
        partial_receiving_attention_item::PartialReceivingAttentionItem,
    },
    repositories::partial_receiving_tracking,
    services::notifications,
};

pub async fn get_attention_required_items(
    pool: &DbPool,
    config: &Config,
) -> Result<Vec<PartialReceivingAttentionItem>, AppError> {
    let items =
        partial_receiving_tracking::find_attention_required(pool)
            .await?;

    for item in &items {
        let mut variables = HashMap::new();

        variables.insert(
            "wso_number".to_string(),
            item.wso_number.clone(),
        );

        variables.insert(
            "description".to_string(),
            item.description.clone(),
        );

        variables.insert(
            "design_code".to_string(),
            item.design_code.clone(),
        );

        variables.insert(
            "fabric_code".to_string(),
            item.fabric_code.clone(),
        );

        variables.insert(
            "first_partial_received_at".to_string(),
            item.first_partial_received_at
                .format("%d %B %Y at %H:%M UTC")
                .to_string(),
        );

        variables.insert(
            "attention_after_days".to_string(),
            item.attention_after_days.to_string(),
        );

        variables.insert(
            "elapsed_days".to_string(),
            item.elapsed_days.to_string(),
        );

        variables.insert(
            "overdue_days".to_string(),
            item.overdue_days.to_string(),
        );

        variables.insert(
            "outstanding_quantity".to_string(),
            item.outstanding_quantity.to_string(),
        );

        let context = NotificationContext {
            event_code: "partial_receiving_attention".to_string(),

            actor_name: "Operations Platform".to_string(),

            actor_email: "System".to_string(),

            variables,
        };

        notifications::dispatch(
            pool,
            config,
            context,
        )
        .await?;

        partial_receiving_tracking::mark_notification_sent(
            pool,
            item.tracking_id,
            Utc::now(),
        )
        .await?;
    }

    Ok(items)
}