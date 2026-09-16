use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::{
        purchase_order::{
            cancel_purchase_order,
            create_purchase_order,
            get_purchase_order,
            get_purchase_orders,
            reactivate_purchase_order,
            update_purchase_order,
            upload_attachment,
        },
        po_item::{add_item, get_item, update_item},
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/purchase-orders",
            post(create_purchase_order)
                .get(get_purchase_orders),
        )

        .route(
            "/purchase-orders/{id}",
            get(get_purchase_order)
                .put(update_purchase_order),
        )

        .route(
            "/purchase-orders/{id}/attachment",
            post(upload_attachment),
        )

        .route(
            "/purchase-orders/{id}/cancel",
            patch(cancel_purchase_order),
        )

        .route(
            "/purchase-orders/{id}/reactivate",
            patch(reactivate_purchase_order),
        )

        // Items are always created against a specific PO,
        // mirroring /wso-items/{id}/line-items' nesting style.
        .route(
            "/purchase-orders/{id}/items",
            post(add_item),
        )

        // Once created, an item is addressed directly — WSO has
        // no equivalent standalone item route; added deliberately
        // here since PO item details are more likely to need
        // correcting after creation.
        .route(
            "/po-items/{id}",
            get(get_item)
                .put(update_item),
        )
}