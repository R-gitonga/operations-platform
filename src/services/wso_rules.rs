use crate::{
    errors::app_error::AppError,
    models::wso::WsoOrder,
};

pub fn ensure_can_receive(
    order: &WsoOrder,
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Cannot receive items into a cancelled Workshop Order."
                .into(),
        ));
    }

    if order.status.eq_ignore_ascii_case("completed") {
        return Err(AppError::BadRequest(
            "This Workshop Order has already been completed."
                .into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_edit(
    order: &WsoOrder,
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Cancelled Workshop Orders cannot be edited."
                .into(),
        ));
    }

    if order.status.eq_ignore_ascii_case("completed") {
        return Err(AppError::BadRequest(
            "Completed Workshop Orders cannot be edited."
                .into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_cancel(
    order: &WsoOrder,
) -> Result<(), AppError> {

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "This Workshop Order is already cancelled."
                .into(),
        ));
    }

    if order.status.eq_ignore_ascii_case("completed") {
        return Err(AppError::BadRequest(
            "Completed Workshop Orders cannot be cancelled."
                .into(),
        ));
    }

    Ok(())
}

pub fn ensure_can_reactivate(
    order: &WsoOrder,
) -> Result<(), AppError> {

    if !order.status.eq_ignore_ascii_case("cancelled") {
        return Err(AppError::BadRequest(
            "Only cancelled Workshop Orders can be reactivated."
                .into(),
        ));
    }

    Ok(())
}