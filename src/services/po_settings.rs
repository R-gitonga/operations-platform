use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{po_settings::PoSettings, update_po_settings::UpdatePoSettings},
    repositories::po_settings,
};

pub async fn get_settings(
    pool: &DbPool,
) -> Result<PoSettings, AppError> {
    Ok(po_settings::get(pool).await?)
}

pub async fn update_settings(
    pool: &DbPool,
    settings: UpdatePoSettings,
) -> Result<PoSettings, AppError> {

    for (label, value) in [
        ("Overdue grace days", settings.overdue_grace_days),
        ("Approaching window days", settings.approaching_window_days),
        ("Stalled-after days", settings.stalled_after_days),
        ("Unresolved defect days", settings.unresolved_defect_after_days),
        ("Alert sweep interval", settings.alert_sweep_interval_minutes),
    ] {
        if value < 0 {
            return Err(AppError::BadRequest(
                format!("{label} cannot be negative."),
            ));
        }
    }

    if settings.alert_sweep_interval_minutes < 1 {
        return Err(AppError::BadRequest(
            "Alert sweep interval must be at least 1 minute.".to_string(),
        ));
    }

    Ok(po_settings::update(pool, &settings).await?)
}