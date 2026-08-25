use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        category::{
            Category,
            CreateCategoryRequest,
            UpdateCategoryRequest,
        },
        wso_item_by_category::WsoItemByCategory,
    },
    repositories::{category, wso_item},
};

fn validate_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        Err(AppError::BadRequest(
            "Category name cannot be empty".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub async fn find_all(pool: &DbPool) -> Result<Vec<Category>, AppError> {
    Ok(category::find_all(pool).await?)
}

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<Category, AppError> {
    Ok(category::find_by_id(pool, id).await?)
}

pub async fn create(
    pool: &DbPool,
    payload: &CreateCategoryRequest,
) -> Result<Category, AppError> {
    validate_name(&payload.name)?;
    Ok(category::create(pool, payload.name.trim()).await?)
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    payload: &UpdateCategoryRequest,
) -> Result<Category, AppError> {
    validate_name(&payload.name)?;
    Ok(category::update(pool, id, payload.name.trim()).await?)
}

pub async fn delete(pool: &DbPool, id: i32) -> Result<Category, AppError> {
    Ok(category::delete(pool, id).await?)
}

pub async fn find_items_by_category(
    pool: &DbPool,
    category_id: i32,
) -> Result<Vec<WsoItemByCategory>, AppError> {
    Ok(wso_item::find_by_category(pool, category_id).await?)
}