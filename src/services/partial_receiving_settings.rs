use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::partial_receiving_settings::PartialReceivingSettings,
    repositories::partial_receiving_settings,
};

pub async fn get_settings(
    pool: &DbPool,
) -> Result<PartialReceivingSettings, AppError> {
    Ok(
        partial_receiving_settings::get(pool)
            .await?
    )
}

pub async fn update_attention_after_days(
    pool: &DbPool,
    days: i32,
) -> Result<PartialReceivingSettings, AppError> {
    if days < 0 {
        return Err(AppError::BadRequest(
            "Attention threshold cannot be negative.".to_string(),
        ));
    }

    Ok(
        partial_receiving_settings::update_attention_after_days(
            pool,
            days,
        )
        .await?
    )
}