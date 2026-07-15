use chrono::Utc;
use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::line_item::{
        CreateWsoLineItemRequest,
        ReceiveLineItemRequest,
        UpdateWsoLineItemRequest,
        WsoLineItem,
    },
    repositories::line_item,
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
        return Err(AppError::BadRequest(
            "qty_raised cannot be negative".to_string(),
        ));
    }

    if qty_received < 0 {
        return Err(AppError::BadRequest(
            "qty_received cannot be negative".to_string(),
        ));
    }

    if qty_received > qty_raised {
        return Err(AppError::BadRequest(
            "qty_received cannot exceed qty_raised".to_string(),
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
    wso_order_id: i32,
    payload: &CreateWsoLineItemRequest,
) -> Result<WsoLineItem, AppError> {
    validate_create_payload(payload)?;
    Ok(line_item::create(pool, wso_order_id, payload).await?)
}

pub async fn find_by_wso(
    pool: &DbPool,
    wso_order_id: i32,
) -> Result<Vec<WsoLineItem>, AppError> {
    Ok(line_item::find_by_wso(pool, wso_order_id).await?)
}

pub async fn find_by_id(
    pool: &DbPool,
    line_item_id: i32,
) -> Result<WsoLineItem, AppError> {
    Ok(line_item::find_by_id(pool, line_item_id).await?)
}

pub async fn update(
    pool: &DbPool,
    line_item_id: i32,
    payload: UpdateWsoLineItemRequest,
) -> Result<WsoLineItem, AppError> {
    let mut item = line_item::find_by_id(pool, line_item_id).await?;

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

    validate_quantities(item.qty_raised, item.qty_received)?;
    Ok(line_item::update(pool, &item).await?)
}

pub async fn receive(
    pool: &DbPool,
    line_item_id: i32,payload: ReceiveLineItemRequest,
) -> Result<WsoLineItem, AppError> {
    if payload.quantity <= 0 {
        return Err(AppError::BadRequest(
            "Quantity must be greater than zero.".to_string(),
        ));
    }

    let mut item = line_item::find_by_id(pool, line_item_id).await?;

    let new_received = item.qty_received + payload.quantity;

    if new_received > item.qty_raised {
        return Err(AppError::BadRequest(
            "Received quantity exceeds quantity raised.".to_string(),
        ));
    }

    item.qty_received = new_received;

    item.balance = item.qty_raised - item.qty_received;

    item.received_date = Some(
        Utc::now().date_naive()
    );

    item.status = if item.qty_received == 0 {
        "Raised".to_string()
    } else if item.balance == 0 {
        "Completed".to_string()
    } else {
        "Partially Received".to_string()
    };

    Ok(
        line_item::update(pool, &item).await?
    )
}

pub async fn delete(
    pool: &DbPool,
    line_item_id: i32,
) -> Result<WsoLineItem, AppError> {
    Ok(line_item::delete(pool, line_item_id).await?)
}
