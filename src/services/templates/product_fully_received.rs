use crate::models::{notification_context::NotificationContext, rendered_email::RenderedEmail};

pub fn render(context: &NotificationContext) -> Result<RenderedEmail, String> {
    let wso_number = context.get("wso_number");
    let req_number = context.get("req_number");
    let description = context.get("description");
    let design_code = context.get("design_code");
    let fabric_code = context.get("fabric_code");

    let html = format!(
        r#"
        <h2>Product Fully Received</h2>

        <p>
            A product has been fully received and has been marked as completed.
        </p>

        <hr>

        <p>
            <strong>WSO Number:</strong> {}
        </p>

        <p>
            <strong>REQ Number:</strong> {}
        </p>

        <p>
            <strong>Product Description:</strong> {}
        </p>

        <p>
            <strong>Design Code:</strong> {}
        </p>

        <p>
            <strong>Fabric Code:</strong> {}
        </p>

        <hr>

        <p>
            <strong>Completed By:</strong> {}
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
        description,
        design_code,
        fabric_code,
        context.actor_name,
        context.actor_email,
    );

    Ok(RenderedEmail {
        subject: format!("Product Fully Received for WSO {}", wso_number),
        html_body: html,
    })
}
