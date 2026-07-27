use crate::models::{
    notification_context::NotificationContext,
    rendered_email::RenderedEmail,
};

pub fn render(
    context: &NotificationContext,
) -> Result<RenderedEmail, String> {

    let wso_number = context.get("wso_number");
    let department = context.get("department");
    let description = context.get("description");

    let html = format!(
        r#"
        <h2>Workshop Order Created</h2>

        <p>A new Workshop Order has been created.</p>

        <hr>

        <p><strong>WSO Number:</strong> {}</p>

        <p><strong>Department:</strong> {}</p>

        <p><strong>Description:</strong> {}</p>

        <hr>

        <p><strong>Created By:</strong> {}</p>

        <p><strong>Email:</strong> {}</p>

        <br>

        <p><em>Generated automatically by the Operations Platform.</em></p>
        "#,
        wso_number,
        department,
        description,
        context.actor_name,
        context.actor_email,
    );

    Ok(RenderedEmail {

        subject: format!(
            "Workshop Order {} Created",
            wso_number,
        ),

        html_body: html,
    })
}