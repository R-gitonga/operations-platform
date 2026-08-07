use crate::{
    errors::app_error::AppError,
    models::production_stage::ProductionStage,
};

pub fn ensure_code_not_empty(
    code: &str,
) -> Result<(), AppError> {

    if code.trim().is_empty() {

        return Err(
            AppError::Validation(
                "Stage code cannot be empty.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_display_name_not_empty(
    name: &str,
) -> Result<(), AppError> {

    if name.trim().is_empty() {

        return Err(
            AppError::Validation(
                "Display name cannot be empty.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_valid_duration(
    duration: Option<i32>,
) -> Result<(), AppError> {

    if let Some(hours) = duration {

        if hours <= 0 {

            return Err(
                AppError::Validation(
                    "Expected duration must be greater than zero.".into(),
                ),
            );
        }
    }

    Ok(())
}

pub fn ensure_stage_is_active(
    stage: &ProductionStage,
) -> Result<(), AppError> {

    if !stage.active {

        return Err(
            AppError::Validation(
                format!(
                    "Production stage '{}' is inactive.",
                    stage.display_name
                ),
            ),
        );
    }

    Ok(())
}

pub fn ensure_stage_is_empty(
    item_count: i64,
) -> Result<(), AppError> {

    if item_count > 0 {

        return Err(
            AppError::Validation(
                format!(
                    "Stage still contains {} product(s). Move them before deactivating it.",
                    item_count
                ),
            ),
        );
    }

    Ok(())
}

pub fn ensure_code_available(
    existing: Option<ProductionStage>,
    current_id: Option<i32>,
) -> Result<(), AppError> {

    if let Some(stage) = existing {

        if Some(stage.id) != current_id {

            return Err(
                AppError::Validation(
                    format!(
                        "A production stage with code '{}' already exists.",
                        stage.code
                    ),
                ),
            );
        }
    }

    Ok(())
}

pub fn validate(
    stage: &ProductionStage,
) -> Result<(), AppError> {

    ensure_code_not_empty(&stage.code)?;

    ensure_display_name_not_empty(
        &stage.display_name,
    )?;

    ensure_valid_duration(
        stage.expected_duration_hours,
    )?;

    Ok(())
}