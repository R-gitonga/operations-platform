use crate::models::notification_context::NotificationContext;

use crate::models::rendered_email::RenderedEmail;

pub fn render(
    context: &NotificationContext,
) -> Result<RenderedEmail, String> {

    let wso_number =
        context.get("wso_number");

    let description =
        context.get("description");

    let design_code =
        context.get("design_code");

    let fabric_code =
        context.get("fabric_code");

    let stage_name =
        context.get("stage_name");

    let stage_started_at =
        context.get("stage_started_at");

    let expected_duration_hours =
        context.get("expected_duration_hours");

    let elapsed_hours =
        context.get("elapsed_hours");

    let overdue_hours =
        context.get("overdue_hours");

    Ok(RenderedEmail {

        subject: format!(
            "Attention Required: WSO {} - {}",
            wso_number,
            description
        ),

        html_body: format!(
            r#"
        <h2>Product Attention Required</h2>

        <p>
            A product has remained in its current production stage
            longer than the expected duration.
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
            <strong>Current Stage:</strong> {}
        </p>

        <p>
            <strong>Stage Started:</strong> {}
        </p>

        <p>
            <strong>Expected Duration:</strong>
            {} hours
        </p>

        <p>
            <strong>Time in Stage:</strong>
            {} hours
        </p>

        <p>
            <strong>Overdue By:</strong>
            {} hours
        </p>

        <br>

        <p>
            Please investigate this product.
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
            stage_name,
            stage_started_at,
            expected_duration_hours,
            elapsed_hours,
            overdue_hours,
        ),
    })
}