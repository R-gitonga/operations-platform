use sqlx::query_as;

use crate::{
    database::DbPool,
    models::po_item_note::PoItemNote,
};

pub async fn create(
    pool: &DbPool,
    po_item_id: i32,
    note: &str,
    created_by: &str,
) -> Result<PoItemNote, sqlx::Error> {
    query_as::<_, PoItemNote>(
        r#"
        INSERT INTO po_item_notes
        (
            po_item_id,
            note,
            created_by
        )
        VALUES
        (
            $1, $2, $3
        )
        RETURNING
            id,
            po_item_id,
            note,
            created_by,
            created_at
        "#,
    )
    .bind(po_item_id)
    .bind(note)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

pub async fn find_by_item(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoItemNote>, sqlx::Error> {
    query_as::<_, PoItemNote>(
        r#"
        SELECT
            id,
            po_item_id,
            note,
            created_by,
            created_at
        FROM po_item_notes
        WHERE po_item_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(po_item_id)
    .fetch_all(pool)
    .await
}