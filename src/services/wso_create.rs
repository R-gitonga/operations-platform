use std::collections::HashMap;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        create_complete_wso::{
            CreateCompleteWsoRequest,
            CreateProductionItemRequest,
        },
        create_wso_item::CreateWsoItemRequest,
        wso_detail::WsoDetail,
        wso_item_detail::WsoItemDetail,
        notification_context::NotificationContext,
    },
    repositories::{
        line_item,
        production_stage,
        wso,
        wso_item,
    },
    services::{
        notifications,
    }
};

use super::line_item as line_item_service;

pub async fn create_complete_wso(
    pool: &DbPool,
    payload: &CreateCompleteWsoRequest,
) -> Result<WsoDetail, AppError> {

    //-------------------------------------------------
    // Validate every line item
    //-------------------------------------------------

    for item in &payload.items {
        for line in &item.line_items {
            line_item_service::validate_create_payload(line)?;
        }
    }

    let mut tx = pool.begin().await?;

    //-------------------------------------------------
    // Create WSO
    //-------------------------------------------------

    let created_wso =
        wso::create_tx(&mut tx, payload).await?;

    let mut detail_items = Vec::new();

    let mut total_qty_raised = 0;
    let mut total_qty_received = 0;
    let mut total_balance = 0;

    let not_started_stage = production_stage::find_by_code_tx(
        &mut tx,
        "NOT_STARTED",
    )
    .await?;

    let not_started_stage = match not_started_stage {
        Some(stage) => stage,
        None => {
            return Err(AppError::BadRequest(
                "Required production stage 'NOT_STARTED' was not found".to_string(),
            ));
        }
    };

    //-------------------------------------------------
    // Create every production item
    //-------------------------------------------------

    for production_item in &payload.items {

        let item_payload = CreateWsoItemRequest {

            category_id: Some(production_item.category_id),

            description: Some(production_item.description.clone()),

            design_code: Some(production_item.design_code.clone()),

            fabric_code: Some(production_item.fabric_code.clone()),

            branding_required: production_item.branding_required,
        };

        let created_item =
            wso_item::create_with_initial_stage_tx(
                &mut tx,
                created_wso.id,
                &item_payload,
                Some(not_started_stage.id),
                Some("System"),
                Some("Initial stage assigned on item creation"),
            )
            .await?;

        let mut created_lines = Vec::new();

        let mut item_qty_raised = 0;
        let mut item_qty_received = 0;
        let mut item_balance = 0;

        //-------------------------------------------------
        // Create every size line
        //-------------------------------------------------

        for line in &production_item.line_items {

            let created =
                line_item::create_tx(
                    &mut tx,
                    created_wso.id,
                    created_item.id,
                    line,
                )
                .await?;

            item_qty_raised += created.qty_raised;
            item_qty_received += created.qty_received;
            item_balance += created.balance;

            created_lines.push(created);
        }

        total_qty_raised += item_qty_raised;
        total_qty_received += item_qty_received;
        total_balance += item_balance;

        detail_items.push(WsoItemDetail {

            id: created_item.id,

            category_id: created_item.category_id,

            description: created_item.description,

            design_code: created_item.design_code,

            fabric_code: created_item.fabric_code,

            branding_required: created_item.branding_required,

            branding_completed: created_item.branding_completed,

            status: created_item.status,

            current_stage_id: created_item.current_stage_id,

            current_stage_name: created_item.current_stage_name,

            current_stage_color: created_item.current_stage_color,

            current_stage_changed_by: created_item.current_stage_changed_by,

            current_stage_changed_at: created_item.current_stage_changed_at,

            current_stage_notes: created_item.current_stage_notes,

            total_qty_raised: item_qty_raised,

            total_qty_received: item_qty_received,

            total_balance: item_balance,

            line_items: created_lines,
        });
    }

    tx.commit().await?;

    // Dispatch WSO created notification

    let mut variables = HashMap::new();

    variables.insert(
        "wso_number".to_string(),
        created_wso.wso_number.clone(),
    );

    if let Some(req_number) = &created_wso.req_number {
        variables.insert(
            "req_number".to_string(),
            req_number.clone(),
        );
    }

    let context = NotificationContext {
        event_code: "wso_created".to_string(),

        actor_name: "Operations Platform".to_string(),

        actor_email: "System".to_string(),

        variables,
    };

    if let Err(error) =
        notifications::dispatch(
            pool,
            context
        ).await
        {
            eprintln!(
                "Failed to dispatch wso_created notification for WSO {}: {}",
                created_wso.wso_number,
                error
            );
        }

    //-------------------------------------------------
    // Return completed detail
    //-------------------------------------------------

    Ok(WsoDetail {

        id: created_wso.id,

        date_signed: created_wso.date_signed,

        wso_number: created_wso.wso_number,

        req_number: created_wso.req_number,

        attachment_name: created_wso.attachment_name,

        attachment_path: created_wso.attachment_path,

        status: created_wso.status,

        total_items: detail_items.len(),

        total_qty_raised,

        total_qty_received,

        total_balance,

        items: detail_items,
    })
}