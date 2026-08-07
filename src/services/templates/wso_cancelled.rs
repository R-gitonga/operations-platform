use crate::models::{
    notification_context::NotificationContext,
    rendered_email::RenderedEmail,
};

pub fn render(
    context: &NotificationContext,
) -> Result<RenderedEmail, String> {

    let wso_number =
        context.get("wso_number");

    let req_number =
        context.get("req_number");

    let html = format!(
        r#"
        <h2>Workshop Order Cancelled</h2>

        <p>
            A Workshop Order has been cancelled.
        </p>

        <hr>

        <p>
            <strong>WSO Number:</strong> {}
        </p>

        <p>
            <strong>REQ Number:</strong> {}
        </p>

        <hr>

        <p>
            <strong>Cancelled By:</strong> {}
        </p>

        <p>
            <strong>Email:</strong> {}
        </p>

        <br>

        <p>
            <em>
                Generated automatically by the Operations Platform.
            </em>
        </p>
        "#,
        wso_number,
        req_number,
        context.actor_name,
        context.actor_email,
    );

    Ok(RenderedEmail {

        subject: format!(
            "Workshop Order {} (REQ {}) Cancelled",
            wso_number,
            req_number,
        ),

        html_body: html,
    })
}