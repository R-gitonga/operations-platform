use crate::{
    database::DbPool, errors::app_error::AppError,
    models::po_dashboard::PoOverdueItem, repositories::po_overdue,
};

pub async fn get_overdue_items(
    pool: &DbPool,
) -> Result<Vec<PoOverdueItem>, AppError> {
    Ok(po_overdue::find_overdue(pool).await?)
}