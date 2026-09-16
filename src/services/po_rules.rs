use crate::{
    errors::app_error::AppError,
    models::purchase_order::PurchaseOrder,
};

pub fn ensure_can_edit(
    order: &PurchaseOrder,
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Cancelled Purchase Orders cannot be edited.".into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_receive(
    order: &PurchaseOrder,
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Cannot receive items into a cancelled Purchase Order.".into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_cancel(
    order: &PurchaseOrder
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "This Purchase Order is already cancelled.".into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_reactivate(
    order: &PurchaseOrder
) -> Result<(), AppError> {

    if !order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Only cancelled Purchase Orders can be reactivated.".into(),
        ));
    }

    Ok(())

}