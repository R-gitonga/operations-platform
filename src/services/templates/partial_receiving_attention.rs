use crate::models::notification_context::NotificationContext;
use crate::models::rendered_email::RenderedEmail;

pub fn render(
    context: &NotificationContext,
) -> Result<RenderedEmail, String> {
    let wso_number = context.get("wso_number");
    let description = context.get("description");
    let design_code = context.get("design_code");
    let fabric_code = context.get("fabric_code");
    let first_partial_received_at =
        context.get("first_partial_received_at");
    let attention_after_days =
        context.get("attention_after_days");
    let elapsed_days =
        context.get("elapsed_days");
    let overdue_days =
        context.get("overdue_days");
    let outstanding_quantity =
        context.get("outstanding_quantity");

    Ok(RenderedEmail {
        subject: format!(
            "Partial Receiving Attention: WSO {} - {}",
            wso_number,
            description
        ),

        html_body: format!(
            r#"
            <h2>Partial Receiving Attention Required</h2>

            <p>
                A product has been partially received but still
                has an outstanding quantity beyond the configured
                attention threshold.
            </p>

            <hr>

            <p>
                <strong>WSO Number:</strong> {}
            </p>

            <p>
                <strong>Product:</strong> {}
            </p>

            <p>
                <strong>Design Code:</strong> {}
            </p>

            <p>
                <strong>Fabric Code:</strong> {}
            </p>

            <hr>

            <p>
                <strong>First Partial Receipt:</strong> {}
            </p>

            <p>
                <strong>Attention Threshold:</strong>
                {} days
            </p>

            <p>
                <strong>Elapsed:</strong>
                {} days
            </p>

            <p>
                <strong>Overdue By:</strong>
                {} days
            </p>

            <p>
                <strong>Outstanding Quantity:</strong>
                {}
            </p>

            <br>

            <p>
                Please follow up on the outstanding quantity.
            </p>

            <br>

            <p>
                <em>
                    Generated automatically by the Operations Platform.
                </em>
            </p>
            "#,
            wso_number,
            description,
            design_code,
            fabric_code,
            first_partial_received_at,
            attention_after_days,
            elapsed_days,
            overdue_days,
            outstanding_quantity,
        ),
    })
}