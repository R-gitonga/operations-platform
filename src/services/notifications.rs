use crate::{
    config::Config,
    database::DbPool,
    models::{
        enqueue_notification_job::EnqueueNotificationJob,
        notification_context::NotificationContext,
        notification_dispatch::NotificationDispatch,
    },
    services::{
        notification_job,
        notification_log,
        notification_recipient,
        settings,
        template_renderer,
    },
};

pub async fn resolve(
    pool: &DbPool,
    event_code: &str,
) -> Result<NotificationDispatch, sqlx::Error> {
    let setting =
        settings::find_by_code(pool, event_code).await?;

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
        notification_recipient::find_enabled_by_event(
            pool,
            setting.notification_event_id,
        )
        .await?;

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
    let dispatch =
        resolve(pool, &context.event_code).await?;

    if dispatch.recipients.is_empty() {
        return Ok(());
    }

    let rendered =
        template_renderer::render(&context)
            .map_err(|error| {
                sqlx::Error::Protocol(error.into())
            })?;

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
    let sender_name =
        config.system_notification_name.clone();

    let sender_email =
        config.system_notification_email.clone();

    for recipient in dispatch.recipients {
        let log =
            notification_log::create_pending(
                pool,
                recipient.notification_event_id,
                &recipient.email,
                "email",
            )
            .await?;

        let job = EnqueueNotificationJob {
            notification_log_id: log.id,

            sender_name: sender_name.clone(),

            sender_email: sender_email.clone(),

            recipient_email: recipient.email.clone(),

            subject: rendered.subject.clone(),

            html_body: rendered.html_body.clone(),
        };

        notification_job::enqueue(
            pool,
            job,
        )
        .await?;
    }

    Ok(())
}