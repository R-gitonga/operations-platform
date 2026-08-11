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
     * Determine the sender based on the origin of
     * the notification.
     *
     * User-driven events are sent using the email
     * address of the user who performed the action.
     *
     * System-driven events use the configured
     * system notification mailbox.
     */
    let (sender_name, sender_email) =
    if context.event_code == "attention_required" {
        (
            config.system_notification_name.clone(),
            config.system_notification_email.clone(),
        )
    } else {
        (
            context.actor_name.clone(),
            context.actor_email.clone(),
        )
    };

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