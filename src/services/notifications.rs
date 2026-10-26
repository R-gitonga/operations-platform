use crate::{
    config::Config,
    database::DbPool,
    models::{
        notification_context::NotificationContext, notification_dispatch::NotificationDispatch,
    },
    services::{notification_recipient, settings, template_renderer},
};

pub async fn resolve(pool: &DbPool, event_code: &str) -> Result<NotificationDispatch, sqlx::Error> {
    let setting = settings::find_by_code(pool, event_code).await?;

    if !setting.enabled {
        return Ok(NotificationDispatch {
            event_code: setting.code,
            event_name: setting.display_name,
            email_enabled: false,
            in_app_enabled: false,
            recipients: Vec::new(),
        });
    }

    let recipients =
        notification_recipient::find_enabled_by_event(pool, setting.notification_event_id).await?;

    Ok(NotificationDispatch {
        event_code: setting.code,
        event_name: setting.display_name,
        email_enabled: setting.email_enabled,
        in_app_enabled: setting.in_app_enabled,
        recipients,
    })
}

pub async fn dispatch(
    pool: &DbPool,
    config: &Config,
    context: NotificationContext,
) -> Result<(), sqlx::Error> {
    let dispatch = resolve(pool, &context.event_code).await?;

    if !dispatch.email_enabled || dispatch.recipients.is_empty() {
        return Ok(());
    }

    let rendered =
        template_renderer::render(&context).map_err(|error| sqlx::Error::Protocol(error.into()))?;

    /*
     * Notifications are always sent from the configured
     * Operations Platform mailbox.
     *
     * The actor remains separate from the sender:
     *
     *   actor_name / actor_email
     *       = user who performed the action
     *
     *   system_notification_name / system_notification_email
     *       = mailbox that sends the notification
     *
     * Actor information remains available through
     * NotificationContext and can be rendered into the
     * notification template.
     */
    let sender_name = config.system_notification_name.clone();

    let sender_email = config.system_notification_email.clone();

    let mut tx = pool.begin().await?;
    let mut seen = std::collections::HashSet::new();
    for recipient in dispatch.recipients {
        let email = recipient.email.trim().to_lowercase();
        if !seen.insert(email.clone()) {
            continue;
        }
        let log_id: i32 = sqlx::query_scalar("INSERT INTO notification_logs(notification_event_id,recipient_email,channel,status) VALUES($1,$2,'email','pending') RETURNING id")
            .bind(recipient.notification_event_id).bind(&email).fetch_one(&mut *tx).await?;
        sqlx::query("INSERT INTO notification_jobs(notification_log_id,sender_name,sender_email,recipient_email,subject,html_body) VALUES($1,$2,$3,$4,$5,$6)")
            .bind(log_id).bind(&sender_name).bind(&sender_email).bind(email)
            .bind(&rendered.subject).bind(&rendered.html_body).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}
