use std::collections::HashMap;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        attention_required_item::AttentionRequiredItem,
        notification_context::NotificationContext,
        production_stage::ProductionStage,
        production_stage_item::ProductionStageItem,
        production_stage_requests::{
            CreateProductionStageRequest,
            UpdateProductionStageRequest,
        },
    },
    repositories::{
        attention_required_notification,
        notification_event,
        notification_setting,
        production_stage,
    },
    services::{
        notifications,
        production_stage_rules,
    },
};

pub async fn list(
    pool: &DbPool,
) -> Result<Vec<ProductionStage>, sqlx::Error> {

    production_stage::find_all(pool).await
}

pub async fn get(
    pool: &DbPool,
    id: i32,
) -> Result<ProductionStage, sqlx::Error> {

    production_stage::find_by_id(pool, id).await
}

pub async fn create(
    pool: &DbPool,
    request: &CreateProductionStageRequest,
) -> Result<ProductionStage, AppError> {

    let stage = ProductionStage {

        id: 0,

        code: request.code.clone(),

        display_name: request.display_name.clone(),

        display_order: request.display_order,

        color: request.color.clone(),

        active: true,

        expected_duration_hours: request.expected_duration_hours,

        attention_enabled: request.attention_enabled,
    };

    production_stage_rules::validate(&stage)?;

    let existing =
        production_stage::find_by_code(
            pool,
            &stage.code,
        )
        .await?;

    production_stage_rules::ensure_code_available(
        existing,
        None,
    )?;

    Ok(
        production_stage::create(
            pool,
            &stage,
        )
        .await?
    )
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    request: &UpdateProductionStageRequest,
) -> Result<ProductionStage, AppError> {

    let mut stage =
        production_stage::find_by_id(
            pool,
            id,
        )
        .await?;

    stage.code =
        request.code.clone();

    stage.display_name =
        request.display_name.clone();

    stage.display_order =
        request.display_order;

    stage.color =
        request.color.clone();

    stage.expected_duration_hours =
        request.expected_duration_hours;

    stage.attention_enabled =
        request.attention_enabled;

    production_stage_rules::validate(
        &stage,
    )?;

    let existing =
        production_stage::find_by_code(
            pool,
            &stage.code,
        )
        .await?;

    production_stage_rules::ensure_code_available(
        existing,
        Some(stage.id),
    )?;

    Ok(
        production_stage::update(
            pool,
            &stage,
        )
        .await?
    )
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<ProductionStage, AppError> {

    let mut stage =
        production_stage::find_by_id(
            pool,
            id,
        )
        .await?;

    production_stage_rules::ensure_stage_is_active(
        &stage,
    )?;

    let item_count =
        production_stage::count_items_in_stage(
            pool,
            id,
        )
        .await?;

    production_stage_rules::ensure_stage_is_empty(
        item_count,
    )?;

    stage.active = false;

    Ok(
        production_stage::update(
            pool,
            &stage,
        )
        .await?
    )
}

pub async fn get_stage_items(
    pool: &DbPool,
    stage_id: i32,
) -> Result<Vec<ProductionStageItem>, sqlx::Error> {

    production_stage::find_items_in_stage(
        pool,
        stage_id,
    )
    .await
}

pub async fn get_attention_required_items(
    pool: &DbPool,
) -> Result<Vec<AttentionRequiredItem>, AppError> {

    let items =
        production_stage::find_attention_required_items(
            pool
        )
        .await?;

    notify_attention_required_items(
        pool,
        &items,
    )
    .await?;

    Ok(items)
}

async fn notify_attention_required_items(
    pool: &DbPool,
    items: &[AttentionRequiredItem],
) -> Result<(), AppError> {

    if items.is_empty() {
        return Ok(());
    }

    let setting =
        notification_setting::find_by_code(
            pool,
            "attention_required",
        )
        .await?;

    if !setting.enabled
        || !setting.email_enabled
    {
        return Ok(());
    }

    let event =
        notification_event::find_by_code(
            pool,
            "attention_required",
        )
        .await?;

    for item in items {

        let already_notified =
            attention_required_notification::has_been_notified(
                pool,
                item.wso_item_id,
                item.current_stage_id,
                item.stage_started_at,
            )
            .await?;

        if already_notified {
            continue;
        }

        let mut variables =
            HashMap::new();

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
            "stage_name".to_string(),
            item.current_stage_name.clone(),
        );

        variables.insert(
            "stage_started_at".to_string(),
            item.stage_started_at
                .format("%d %B %Y at %H:%M UTC")
                .to_string(),
        );

        variables.insert(
            "expected_duration_hours".to_string(),
            item.expected_duration_hours
                .to_string(),
        );

        variables.insert(
            "elapsed_hours".to_string(),
            format!(
                "{:.1}",
                item.elapsed_hours
            ),
        );

        variables.insert(
            "overdue_hours".to_string(),
            format!(
                "{:.1}",
                item.overdue_hours
            ),
        );

        let context =
            NotificationContext {

                event_code:
                    "attention_required"
                    .to_string(),

                actor_name:
                    "Operations Platform"
                    .to_string(),

                actor_email:
                    "System"
                    .to_string(),

                variables,
            };

        notifications::dispatch(
            pool,
            context,
        )
        .await?;

        attention_required_notification::record_notification(
            pool,
            item.wso_item_id,
            item.current_stage_id,
            item.stage_started_at,
            event.id,
        )
        .await?;
    }

    Ok(())
}