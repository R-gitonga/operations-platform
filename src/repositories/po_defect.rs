use sqlx::query_as;

use crate::{
    database::DbPool,
    models::po_defect::PoDefect,
};

const PO_DEFECT_SELECT: &str = r#"
SELECT
    id,
    po_line_item_id,
    qty_defective,
    reason,
    status,
    resolution_type,
    resolution_notes,
    reported_by,
    reported_at,
    resolved_by,
    resolved_at
FROM po_defects
"#;

pub async fn create(
    pool: &DbPool,
    po_line_item_id: i32,
    qty_defective: i32,
    reason: &str,
    reported_by: &str,
) -> Result<PoDefect, sqlx::Error> {
    query_as::<_, PoDefect>(
        r#"
        INSERT INTO po_defects
        (
            po_line_item_id,
            qty_defective,
            reason,
            reported_by
        )
        VALUES
        (
            $1, $2, $3, $4
        )
        RETURNING
            id,
            po_line_item_id,
            qty_defective,
            reason,
            status,
            resolution_type,
            resolution_notes,
            reported_by,
            reported_at,
            resolved_by,
            resolved_at
        "#,
    )
    .bind(po_line_item_id)
    .bind(qty_defective)
    .bind(reason)
    .bind(reported_by)
    .fetch_one(pool)
    .await
}

pub async fn resolve(
    pool: &DbPool,
    id: i32,
    resolution_type: &str,
    resolution_notes: Option<&str>,
    resolved_by: &str,
) -> Result<PoDefect, sqlx::Error> {
    query_as::<_, PoDefect>(
        r#"
        UPDATE po_defects
        SET
            status = 'resolved',
            resolution_type = $2,
            resolution_notes = $3,
            resolved_by = $4,
            resolved_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            po_line_item_id,
            qty_defective,
            reason,
            status,
            resolution_type,
            resolution_notes,
            reported_by,
            reported_at,
            resolved_by,
            resolved_at
        "#,
    )
    .bind(id)
    .bind(resolution_type)
    .bind(resolution_notes)
    .bind(resolved_by)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<PoDefect>, sqlx::Error> {
    query_as::<_, PoDefect>(
        &format!("{PO_DEFECT_SELECT} WHERE id = $1"),
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_line_item(
    pool: &DbPool,
    po_line_item_id: i32,
) -> Result<Vec<PoDefect>, sqlx::Error> {
    query_as::<_, PoDefect>(
        &format!("{PO_DEFECT_SELECT} WHERE po_line_item_id = $1 ORDER BY reported_at DESC"),
    )
    .bind(po_line_item_id)
    .fetch_all(pool)
    .await
}

// All defective quantity ever reported against this line, regardless
// of resolution -- used to validate a new report never exceeds what
// was physically delivered.
pub async fn total_defective_for_line(
    pool: &DbPool,
    po_line_item_id: i32,
) -> Result<i32, sqlx::Error> {
    let total: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(qty_defective), 0) FROM po_defects WHERE po_line_item_id = $1",
    )
    .bind(po_line_item_id)
    .fetch_one(pool)
    .await?;

    Ok(total as i32)
}