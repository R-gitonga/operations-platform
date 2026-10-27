use crate::{config::Config, database::DbPool, models::rendered_email::RenderedEmail};
use sqlx::Row;

pub fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn render(
    code: &str,
    data: &serde_json::Value,
    base_url: &str,
) -> Result<RenderedEmail, String> {
    let title = match code {
        "po_created" => "Purchase order created",
        "po_cancelled" => "Purchase order cancelled",
        "po_reactivated" => "Purchase order reactivated",
        "po_completed" => "Purchase order completed",
        "po_defect_reported" => "Purchase order defect reported",
        "po_branding_goods_ready" => "Purchase order goods ready for branding",
        "po_delivery_approaching" => "Purchase order delivery approaching",
        "po_delivery_overdue" => "Purchase order delivery overdue",
        "po_partial_stalled" => "Purchase order partial delivery stalled",
        "po_defect_unresolved" => "Purchase order defect unresolved",
        _ => return Err(format!("Unknown PO email event: {code}")),
    };
    let value = |key: &str| escape(data[key].as_str().unwrap_or("—"));
    let reference = data["accounts_reference"].as_str().unwrap_or("—");
    let mut body = format!("<h1>{title}</h1><p>Accounts reference: {}<br>ERP reference: {}<br>Supplier: {}<br>Created by: {}</p>",
        value("accounts_reference"), value("erp_reference"), value("supplier"), value("created_by"));
    let paid = data["is_paid_order"].as_bool() == Some(true);
    if paid {
        body.push_str(&format!("<h2>Paid order</h2><p>Branch: {}</p>", value("branch_name")));
    }
    if let Some(actor) = data["actor"].as_str() {
        body.push_str(&format!("<p>Action by: {}</p>", escape(actor)));
    }
    if let Some(description) = data["description"].as_str() {
        body.push_str(&format!("<p>{}</p>", escape(description)));
    }
    for (key, label) in [
        ("window_days", "Approaching window (days)"),
        ("grace_days", "Grace (days)"),
        ("days_overdue", "Days past expected delivery"),
        ("threshold_days", "Alert threshold (24-hour days)"),
        ("last_receipt_at", "Last receipt"),
    ] {
        if !data[key].is_null() {
            body.push_str(&format!(
                "<p>{label}: {}</p>",
                escape(
                    &data[key]
                        .as_str()
                        .map(str::to_string)
                        .unwrap_or_else(|| data[key].to_string())
                )
            ));
        }
    }
    if let Some(items) = data["items"].as_array() {
        for item in items {
            if let Some(reason) = item["paid_order_reason"].as_str() {
                let label = if reason == "mtm" { "Make to measure (MTM)" } else { "Stock-out" };
                body.push_str(&format!("<p>Item reason: {label}</p>"));
            }
            if item["branding_required"].as_bool() == Some(true) {
                body.push_str(&format!(
                    "<p>Branding: {} · Location: {}</p>",
                    escape(item["branding_type"].as_str().unwrap_or("Required")),
                    escape(item["branding_location"].as_str().unwrap_or("Not set"))
                ));
            }
            body.push_str(&format!("<h2>{}</h2><p>Expected delivery: {}</p><table><tr><th>Size</th><th>Ordered</th><th>Delivered</th><th>Accepted</th><th>Outstanding</th></tr>",
                escape(item["description"].as_str().unwrap_or("Product")),escape(item["expected_delivery_date"].as_str().unwrap_or("Not set"))));
            if let Some(lines) = item["lines"].as_array() {
                for line in lines {
                    body.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                        escape(line["size"].as_str().unwrap_or("—")),
                        line["ordered"],
                        line["delivered"],
                        line["accepted"],
                        line["outstanding"]
                    ));
                }
            }
            body.push_str("</table>");
        }
    }
    if data["defect"].is_object() {
        let d = &data["defect"];
        body.push_str(&format!(
            "<p>Defect #{} · Quantity: {}<br>Reason: {}<br>Reported by: {}<br>Reported at: {}</p>",
            d["id"],
            d["qty_defective"],
            escape(d["reason"].as_str().unwrap_or("—")),
            escape(d["reported_by"].as_str().unwrap_or("—")),
            escape(d["reported_at"].as_str().unwrap_or("—"))
        ));
    }
    let url = format!(
        "{}/purchase-orders/{}",
        base_url.trim_end_matches('/'),
        data["purchase_order_id"].as_i64().ok_or("Missing PO id")?
    );
    if !base_url.starts_with("https://") && !base_url.starts_with("http://") {
        return Err("FRONTEND_BASE_URL must use http or https".into());
    }
    body.push_str(&format!("<p><a href=\"{}\">View purchase order</a></p><p>Quantities reflect the event snapshot. Delivery deadlines use Africa/Nairobi time.</p>",escape(&url)));
    Ok(RenderedEmail {
        subject: format!("{}{title}: {}", if paid { "[Paid order] " } else { "" }, reference.replace(['\r', '\n'], " ")),
        html_body: body,
    })
}

/// Occurrence, logs and recipient jobs commit together. Multiple workers skip
/// locked occurrences; normalized recipient addresses prevent duplicate delivery.
pub async fn enqueue_occurrences(pool: &DbPool, config: &Config) -> Result<(), sqlx::Error> {
    for _ in 0..100 {
        let mut tx = pool.begin().await?;
        let row = sqlx::query("SELECT id,event_code,occurrence_key,payload::text FROM po_notification_occurrences WHERE state='pending' ORDER BY id FOR UPDATE SKIP LOCKED LIMIT 1")
            .fetch_optional(&mut *tx).await?;
        let Some(row) = row else {
            break;
        };
        let id: i64 = row.get("id");
        let code: String = row.get("event_code");
        let key: String = row.get("occurrence_key");
        let timed = matches!(
            code.as_str(),
            "po_delivery_approaching"
                | "po_delivery_overdue"
                | "po_partial_stalled"
                | "po_defect_unresolved"
        );
        let eligible: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM notification_events e JOIN notification_settings s ON s.notification_event_id=e.id WHERE e.code=$1 AND s.enabled AND s.email_enabled)")
            .bind(&code).fetch_one(&mut *tx).await?;
        let current: bool = !timed
            || sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM po_email_timed_candidates WHERE key=$1 AND qualifies)",
            )
            .bind(&key)
            .fetch_one(&mut *tx)
            .await?;
        let recipients: Vec<(i32,String)> = sqlx::query_as("SELECT DISTINCT r.notification_event_id,lower(btrim(r.email)) FROM notification_recipients r JOIN notification_events e ON e.id=r.notification_event_id WHERE e.code=$1 AND r.enabled")
            .bind(&code).fetch_all(&mut *tx).await?;
        if !eligible || !current || recipients.is_empty() {
            sqlx::query("UPDATE po_notification_occurrences SET state='suppressed' WHERE id=$1")
                .bind(id)
                .execute(&mut *tx)
                .await?;
            tx.commit().await?;
            continue;
        }
        let data: serde_json::Value = serde_json::from_str(row.get::<&str, _>("payload"))
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let rendered =
            render(&code, &data, &config.frontend_base_url).map_err(sqlx::Error::Protocol)?;
        for (event_id, email) in recipients {
            let log_id: i32 = sqlx::query_scalar("INSERT INTO notification_logs(notification_event_id,recipient_email,channel,status) VALUES($1,$2,'email','pending') RETURNING id")
                .bind(event_id).bind(&email).fetch_one(&mut *tx).await?;
            sqlx::query("INSERT INTO notification_jobs(notification_log_id,sender_name,sender_email,recipient_email,subject,html_body,occurrence_id) VALUES($1,$2,$3,$4,$5,$6,$7)")
                .bind(log_id).bind(&config.system_notification_name).bind(&config.system_notification_email).bind(email)
                .bind(&rendered.subject).bind(&rendered.html_body).bind(id).execute(&mut *tx).await?;
        }
        sqlx::query("UPDATE po_notification_occurrences SET state='queued' WHERE id=$1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        if timed {
            sqlx::query("INSERT INTO po_email_timed_history(occurrence_key) VALUES($1) ON CONFLICT DO NOTHING").bind(&key).execute(&mut *tx).await?;
        }
        tx.commit().await?;
    }
    Ok(())
}

pub async fn sweep(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT po_email_sweep()").execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn renders_every_event_and_escapes_untrusted_text() {
        let data = serde_json::json!({"purchase_order_id":42,"accounts_reference":"PO\r\n123","supplier":"<script>&\"", "items":[{"description":"<img>","lines":[]}],"defect":{"reason":"<script>"}});
        for event in [
            "po_created",
            "po_cancelled",
            "po_reactivated",
            "po_completed",
            "po_defect_reported",
            "po_branding_goods_ready",
            "po_delivery_approaching",
            "po_delivery_overdue",
            "po_partial_stalled",
            "po_defect_unresolved",
        ] {
            let email = render(event, &data, "https://example.test/").unwrap();
            assert!(email
                .html_body
                .contains("https://example.test/purchase-orders/42"));
            assert!(!email.html_body.contains("<script>"));
            assert!(!email.subject.contains(['\r', '\n']));
        }
    }
}
