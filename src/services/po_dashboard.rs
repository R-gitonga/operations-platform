use std::collections::HashMap;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::po_dashboard::{
        PoDashboardSummary, PoOrderSummary, PoOutstandingOrder, PoQuantitySummary,
        PoRecentOrder, PoSupplierSummary,
    },
    repositories::{po_dashboard, po_status as po_status_repo},
    services::po_status as po_status_service,
};

const RECENT_ORDERS_LIMIT: i64 = 5;
const OUTSTANDING_LIMIT: usize = 5;

pub async fn get_dashboard(
    pool: &DbPool,
    page: i64,
    page_size: i64,
) -> Result<PoDashboardSummary, AppError> {

    let all_inputs = po_status_repo::find_all_status_inputs(pool).await?;

    // Order/quantity summaries, supplier breakdown, and the largest
    // outstanding POs are all derived in-memory from this one
    // fetched dataset, since deriving status per PO is a pure Rust
    // function (services::po_status::derive_status) rather than a
    // stored column that SQL could GROUP BY directly.

    let mut status_counts: HashMap<String, i64> = HashMap::new();
    let mut total_qty_ordered: i64 = 0;
    let mut total_qty_delivered: i64 = 0;
    let mut total_qty_defective: i64 = 0;

    struct OutstandingCandidate {
        id: i32,
        accounts_reference: String,
        supplier_name: String,
        outstanding_qty: i64,
    }

    let mut outstanding_candidates: Vec<OutstandingCandidate> = Vec::new();

    // supplier_id -> (name, open_orders, total_qty_ordered)
    let mut supplier_totals: HashMap<i32, (String, i64, i64)> = HashMap::new();

    for inputs in &all_inputs {

        let status = po_status_service::derive_status(inputs);

        *status_counts.entry(status.clone()).or_insert(0) += 1;

        total_qty_ordered += inputs.total_ordered;
        total_qty_delivered += inputs.total_delivered;
        total_qty_defective += inputs.total_non_accepted_defective;

        let total_accepted =
            inputs.total_delivered - inputs.total_non_accepted_defective;

        let outstanding_qty = inputs.total_ordered - total_accepted;

        if status != "cancelled" && outstanding_qty > 0 {
            outstanding_candidates.push(OutstandingCandidate {
                id: inputs.purchase_order_id,
                accounts_reference: inputs.accounts_reference.clone(),
                supplier_name: inputs.supplier_name.clone(),
                outstanding_qty,
            });
        }

        let entry = supplier_totals
            .entry(inputs.supplier_id)
            .or_insert_with(|| (inputs.supplier_name.clone(), 0, 0));

        if status != "cancelled" {
            entry.1 += 1;
        }

        entry.2 += inputs.total_ordered;
    }

    let total_qty_accepted = total_qty_delivered - total_qty_defective;
    let total_outstanding = total_qty_ordered - total_qty_accepted;

    outstanding_candidates.sort_by(|a, b| b.outstanding_qty.cmp(&a.outstanding_qty));
    outstanding_candidates.truncate(OUTSTANDING_LIMIT);

    let largest_outstanding = outstanding_candidates
        .into_iter()
        .map(|o| PoOutstandingOrder {
            id: o.id,
            accounts_reference: o.accounts_reference,
            supplier_name: o.supplier_name,
            outstanding_qty: o.outstanding_qty,
        })
        .collect();

    let mut by_supplier: Vec<PoSupplierSummary> = supplier_totals
        .into_iter()
        .map(|(supplier_id, (supplier_name, open_orders, total_qty_ordered))| {
            PoSupplierSummary {
                supplier_id,
                supplier_name,
                open_orders,
                total_qty_ordered,
            }
        })
        .collect();

    by_supplier.sort_by(|a, b| {
        b.open_orders
            .cmp(&a.open_orders)
            .then_with(|| a.supplier_name.cmp(&b.supplier_name))
    });

    // Recent Purchase Orders -- fetched separately since
    // find_all_status_inputs doesn't carry created_at.

    let recent_raw =
        po_dashboard::find_recent_orders(pool, RECENT_ORDERS_LIMIT).await?;

    let mut recent_orders = Vec::with_capacity(recent_raw.len());

    for order in recent_raw {
        let derived_status =
            po_status_service::get_status_for_order(pool, order.id).await?;

        recent_orders.push(PoRecentOrder {
            id: order.id,
            accounts_reference: order.accounts_reference,
            supplier_name: order.supplier_name,
            derived_status,
        });
    }

    let recent_activity =
        po_dashboard::get_recent_activity(pool, page, page_size).await?;

    Ok(PoDashboardSummary {
        orders: PoOrderSummary {
            total: all_inputs.len() as i64,
            active: *status_counts.get("active").unwrap_or(&0),
            partial: *status_counts.get("partial").unwrap_or(&0),
            completed: *status_counts.get("completed").unwrap_or(&0),
            cancelled: *status_counts.get("cancelled").unwrap_or(&0),
        },
        quantities: PoQuantitySummary {
            qty_ordered: total_qty_ordered,
            qty_delivered: total_qty_delivered,
            qty_accepted: total_qty_accepted,
            qty_defective: total_qty_defective,
            outstanding: total_outstanding,
        },
        by_supplier,
        recent_orders,
        largest_outstanding,
        recent_activity,
    })
}