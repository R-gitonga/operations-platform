use crate::{
    models::{notification_context::NotificationContext, rendered_email::RenderedEmail},
    services::templates,
};

pub fn render(context: &NotificationContext) -> Result<RenderedEmail, String> {
    match context.event_code.as_str() {
        "attention_required" => templates::attention_required::render(context),

        "wso_created" => templates::wso_created::render(context),

        "wso_cancelled" => templates::wso_cancelled::render(context),

        "wso_reactivated" => templates::wso_reactivated::render(context),

        "wso_completed" => templates::wso_completed::render(context),

        "product_fully_received" => templates::product_fully_received::render(context),

        "partial_receiving_attention" => {
            templates::partial_receiving_attention::render(context)
        }

        _ => Err(format!(
            "No template registered for '{}'",
            context.event_code,
        )),
    }
}
