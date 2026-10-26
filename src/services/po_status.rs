use std::collections::HashMap;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::po_summary::PoSummary,
    repositories::po_status::{self, PoStatusInputs},
};

/// Status is derived, never persisted beyond active/cancelled. Every size line
/// must be fulfilled for completion, matching PO email readiness. Accepted
/// quantity subtracts every defect except those resolved as accepted.
pub fn derive_status(inputs: &PoStatusInputs) -> String {
    if inputs.raw_status.eq_ignore_ascii_case("cancelled") {
        return "cancelled".to_string();
    }

    let total_accepted = inputs.total_delivered - inputs.total_non_accepted_defective;

    if inputs.total_ordered == 0 || total_accepted <= 0 {
        return "active".to_string();
    }

    if !inputs.all_lines_complete {
        "partial".to_string()
    } else {
        "completed".to_string()
    }
}

pub async fn get_status_for_order(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<String, AppError> {
    let inputs = po_status::find_status_inputs_by_id(pool, purchase_order_id).await?;

    Ok(derive_status(&inputs))
}

pub async fn get_po_summary(pool: &DbPool) -> Result<PoSummary, AppError> {
    let all_inputs = po_status::find_all_status_inputs(pool).await?;

    let mut status_counts: HashMap<String, i64> = HashMap::new();
    let mut total_orders = 0;
    let mut total_qty_ordered = 0;
    let mut total_qty_delivered = 0;
    let mut total_qty_defective = 0;

    for inputs in &all_inputs {
        let status = derive_status(inputs);

        *status_counts.entry(status).or_insert(0) += 1;

        total_orders += 1;
        total_qty_ordered += inputs.total_ordered;
        total_qty_delivered += inputs.total_delivered;
        total_qty_defective += inputs.total_non_accepted_defective;
    }

    let total_qty_accepted = total_qty_delivered - total_qty_defective;

    Ok(PoSummary {
        total_orders,
        status_counts,
        total_qty_ordered,
        total_qty_delivered,
        total_qty_accepted,
        total_qty_defective,
        total_outstanding: total_qty_ordered - total_qty_accepted,
    })
}
