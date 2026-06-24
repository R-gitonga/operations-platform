use crate::{
    database::DbPool,
    models::{
        create_complete_wso::CreateCompleteWsoRequest,
        wso_detail::WsoDetail,
    },
    repositories::{
        line_item,
        wso,
    },
};

pub async fn create_complete_wso(
    pool: &DbPool,
    payload: &CreateCompleteWsoRequest,
) -> Result<WsoDetail, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let created_wso = wso::create_tx(&mut tx, payload).await?;

    let mut created_line_items = Vec::with_capacity(payload.line_items.len());
    for item_payload in &payload.line_items {
        let created_item = line_item::create_tx(&mut tx, created_wso.id, item_payload).await?;
        created_line_items.push(created_item);
    }

    let total_quantity: i32 = created_line_items.iter().map(|item| item.quantity).sum();
    let line_item_count = created_line_items.len();

    let wso_detail = WsoDetail {
        id: created_wso.id,
        wso_number: created_wso.wso_number,
        req_number: created_wso.req_number,
        description: created_wso.description,
        remarks: created_wso.remarks,
        status: created_wso.status,
        line_item_count,
        total_quantity,
        line_items: created_line_items,
    };

    tx.commit().await?;
    Ok(wso_detail)
}