use chrono::Utc;
use std::collections::HashMap;

use crate::{
    config::Config,
    database::DbPool,
    errors::app_error::AppError,
    models::line_item::{
        CreateWsoLineItemRequest, ReceiveLineItemRequest, UpdateWsoLineItemRequest, WsoLineItem,
    },
    models::user::User,
    repositories::{line_item, wso, wso_item},
    services::{notifications, wso as wso_service, wso_rules},
};

const VALID_STATUSES: [&str; 10] = [
    "Raised",
    "Approved",
    "Cutting",
    "Stitching",
    "Printing",
    "Ready",
    "Partially Received",
    "Completed",
    "Cancelled",
    "No Fabric",
];

fn validate_status(status: &str) -> Result<(), AppError> {
    if VALID_STATUSES.contains(&status) {
        Ok(())
    } else {
        Err(AppError::BadRequest(format!(
            "Invalid line item status '{}'",
            status
        )))
    }
}

fn validate_quantities(qty_raised: i32, qty_received: i32) -> Result<(), AppError> {
    if qty_raised < 0 {
        return Err(AppError::BadRequest("qty_raised cannot be negative".into()));
    }

    if qty_received < 0 {
        return Err(AppError::BadRequest(
            "qty_received cannot be negative".into(),
        ));
    }

    if qty_received > qty_raised {
        return Err(AppError::BadRequest(
            "qty_received cannot exceed qty_raised".into(),
        ));
    }

    Ok(())
}

pub fn validate_create_payload(payload: &CreateWsoLineItemRequest) -> Result<(), AppError> {
    let qty_received = payload.qty_received.unwrap_or(0);

    validate_quantities(payload.qty_raised, qty_received)?;

    if let Some(status) = &payload.status {
        validate_status(status)?;
    }

    Ok(())
}

pub async fn create(
    pool: &DbPool,
    config: &Config,
    wso_item_id: i32,
    payload: &CreateWsoLineItemRequest,
    actor: &User,
) -> Result<WsoLineItem, AppError> {
    validate_create_payload(payload)?;

    let production_item = wso_item::find_by_id(pool, wso_item_id).await?;

    let order = wso::find_by_id(pool, production_item.wso_order_id).await?;

    wso_rules::ensure_can_edit(&order)?;

    let created = line_item::create(pool, wso_item_id, payload).await?;

    wso_service::refresh_wso_status(
        pool, 
        config, 
        production_item.wso_order_id, 
        actor).await?;

    Ok(created)
}

pub async fn find_by_wso(pool: &DbPool, wso_item_id: i32) -> Result<Vec<WsoLineItem>, AppError> {
    Ok(line_item::find_by_item(pool, wso_item_id).await?)
}

pub async fn find_by_id(pool: &DbPool, line_item_id: i32) -> Result<WsoLineItem, AppError> {
    Ok(line_item::find_by_id(pool, line_item_id).await?)
}

pub async fn update(
    pool: &DbPool,
    config: &Config,
    line_item_id: i32,
    payload: UpdateWsoLineItemRequest,
    actor: &User,
) -> Result<WsoLineItem, AppError> {
    let mut item = line_item::find_by_id(pool, line_item_id).await?;

    let production_item = wso_item::find_by_id(pool, item.wso_item_id).await?;

    let order = wso::find_by_id(pool, production_item.wso_order_id).await?;

    wso_rules::ensure_can_edit(&order)?;

    if let Some(size) = payload.size {
        item.size = size;
    }

    if let Some(qty_raised) = payload.qty_raised {
        item.qty_raised = qty_raised;
    }

    if let Some(qty_received) = payload.qty_received {
        item.qty_received = qty_received;
    }

    if let Some(received_date) = payload.received_date {
        item.received_date = Some(received_date);
    }

    if let Some(status) = payload.status {
        validate_status(&status)?;
        item.status = status;
    }

    item.balance = item.qty_raised - item.qty_received;

    validate_quantities(item.qty_raised, item.qty_received)?;

    let updated = line_item::update(pool, &item).await?;

    wso_service::refresh_wso_status(
        pool, 
        config,
        production_item.wso_order_id, 
        actor).await?;

    Ok(updated)
}

pub async fn receive(
    pool: &DbPool,
    config: &Config,
    line_item_id: i32,
    payload: ReceiveLineItemRequest,
    actor: &User,
) -> Result<WsoLineItem, AppError> {
    if payload.quantity <= 0 {
        return Err(AppError::BadRequest(
            "Quantity must be greater than zero.".into(),
        ));
    }

    let mut item = line_item::find_by_id(pool, line_item_id).await?;

    let production_item = wso_item::find_by_id(pool, item.wso_item_id).await?;

    let order = wso::find_by_id(pool, production_item.wso_order_id).await?;

    wso_rules::ensure_can_receive(&order)?;

    let new_received = item.qty_received + payload.quantity;

    if new_received > item.qty_raised {
        return Err(AppError::BadRequest(
            "Received quantity exceeds quantity raised.".into(),
        ));
    }

    item.qty_received = new_received;

    item.balance = item.qty_raised - item.qty_received;

    item.received_date = Some(Utc::now().date_naive());

    item.status = if item.balance == 0 {
        "Completed".to_string()
    } else if item.qty_received > 0 {
        "Partially Received".to_string()
    } else {
        "Raised".to_string()
    };

    let updated = line_item::update(pool, &item).await?;

    wso_service::refresh_wso_status(
        pool, 
        config,
        production_item.wso_order_id, 
        actor).await?;

    maybe_notify_product_fully_received(
        pool, 
        config,
        &production_item, 
        &order, 
        actor).await?;

    Ok(updated)
}

async fn maybe_notify_product_fully_received(
    pool: &DbPool,
    config: &Config,
    production_item: &crate::models::wso_item::WsoItem,
    order: &crate::models::wso::WsoOrder,
    actor: &User,
) -> Result<(), AppError> {
    let line_items = line_item::find_by_item(pool, production_item.id).await?;

    let total_balance: i32 = line_items.iter().map(|line| line.balance).sum();

    if total_balance != 0 || production_item.status.eq_ignore_ascii_case("Completed") {
        return Ok(());
    }

    let mut updated_item = production_item.clone();
    updated_item.status = "Completed".to_string();
    wso_item::update(pool, &updated_item).await?;

    let mut variables = HashMap::new();
    variables.insert("wso_number".to_string(), order.wso_number.clone());
    variables.insert(
        "req_number".to_string(),
        order.req_number.clone().unwrap_or_default(),
    );
    variables.insert(
        "description".to_string(),
        production_item.description.clone().unwrap_or_default(),
    );
    variables.insert(
        "design_code".to_string(),
        production_item.design_code.clone().unwrap_or_default(),
    );
    variables.insert(
        "fabric_code".to_string(),
        production_item.fabric_code.clone().unwrap_or_default(),
    );

    let context = crate::models::notification_context::NotificationContext {
        event_code: "product_fully_received".to_string(),
        actor_name: actor.name.clone(),
        actor_email: actor.email.clone(),
        variables,
    };

    notifications::dispatch(
        pool, 
        config,
        context).await?;

    Ok(())
}

pub async fn delete(
    pool: &DbPool,
    config: &Config,
    line_item_id: i32,
    actor: &User,
) -> Result<WsoLineItem, AppError> {
    let item = line_item::find_by_id(pool, line_item_id).await?;

    let production_item = wso_item::find_by_id(pool, item.wso_item_id).await?;

    let order = wso::find_by_id(pool, production_item.wso_order_id).await?;

    wso_rules::ensure_can_edit(&order)?;

    let deleted = line_item::delete(pool, line_item_id).await?;

    wso_service::refresh_wso_status(
        pool, 
        config,
        production_item.wso_order_id, 
        actor).await?;

    Ok(deleted)
}
