use axum::{
    extract::State,
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::partial_receiving_attention_item::PartialReceivingAttentionItem,
    services::partial_receiving_attention,
};

pub async fn get_attention_required_partial_receipts(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<PartialReceivingAttentionItem>>, AppError> {
    let items =
        partial_receiving_attention::get_attention_required_items(
            &state.pool,
            &state.config,
        )
        .await?;

    Ok(Json(items))
}