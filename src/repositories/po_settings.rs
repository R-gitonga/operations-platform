use crate::{
    database::DbPool,
    models::{po_settings::PoSettings, update_po_settings::UpdatePoSettings},
};

pub async fn get(
    pool: &DbPool,
) -> Result<PoSettings, sqlx::Error> {
    sqlx::query_as::<_, PoSettings>(
        r#"
        SELECT
            id,
            overdue_grace_days,
            approaching_window_days,
            stalled_after_days,
            unresolved_defect_after_days,
            alert_sweep_interval_minutes,
            updated_at
        FROM po_settings
        WHERE id = 1
        "#,
    )
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    settings: &UpdatePoSettings,
) -> Result<PoSettings, sqlx::Error> {
    sqlx::query_as::<_, PoSettings>(
        r#"
        UPDATE po_settings
        SET
            overdue_grace_days = $1,
            approaching_window_days = $2,
            stalled_after_days = $3,
            unresolved_defect_after_days = $4,
            alert_sweep_interval_minutes = $5,
            updated_at = NOW()
        WHERE id = 1
        RETURNING
            id,
            overdue_grace_days,
            approaching_window_days,
            stalled_after_days,
            unresolved_defect_after_days,
            alert_sweep_interval_minutes,
            updated_at
        "#,
    )
    .bind(settings.overdue_grace_days)
    .bind(settings.approaching_window_days)
    .bind(settings.stalled_after_days)
    .bind(settings.unresolved_defect_after_days)
    .bind(settings.alert_sweep_interval_minutes)
    .fetch_one(pool)
    .await
}