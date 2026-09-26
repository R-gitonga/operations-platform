use crate::{
    database::DbPool,
    models::po_procurement_event::PoProcurementEvent,
    repositories::po_procurement_event,
};

pub async fn list(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoProcurementEvent>, sqlx::Error> {
    po_procurement_event::find_by_po_item(pool, po_item_id).await
}