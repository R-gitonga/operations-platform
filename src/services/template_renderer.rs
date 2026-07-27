use crate::{
    models::{
        notification_context::NotificationContext,
        rendered_email::RenderedEmail,
    },
    services::templates,
};

pub fn render(
    context: &NotificationContext,
) -> Result<RenderedEmail, String> {

    match context.event_code.as_str() {

        "wso_created" => {
            templates::wso_created::render(context)
        }

        _ => Err(format!(
            "No template registered for '{}'",
            context.event_code,
        )),
    }
}