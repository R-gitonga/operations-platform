use crate::{
    models::email_message::EmailMessage,
    services::{
        email_sender,
        notification_job,
        notification_log,
    },
    database::DbPool,
};

pub async fn process_pending_jobs(
    pool: &DbPool,
) -> Result<(), sqlx::Error> {

    let jobs = notification_job::find_pending(pool).await?;

    for job in jobs {

        let message = EmailMessage {

            to: job.recipient_email.clone(),

            subject: job.subject.clone(),

            html_body: job.html_body.clone(),
        };

        match email_sender::send(message).await {

            Ok(_) => {

                notification_job::mark_sent(
                    pool,
                    job.id,
                )
                .await?;

                notification_log::mark_sent(
                    pool,
                    job.notification_log_id,
                )
                .await?;
            }

            Err(error) => {

                notification_job::mark_failed(
                    pool,
                    job.id,
                    &error,
                )
                .await?;

                notification_log::mark_failed(
                    pool,
                    job.notification_log_id,
                    &error,
                )
                .await?;
            }
        }
    }

    Ok(())
}