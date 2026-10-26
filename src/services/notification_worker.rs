use crate::{
    config::Config,
    database::DbPool,
    models::{email_message::EmailMessage, notification_job::NotificationJob},
    services::{email_sender, po_notifications},
};

/// Claim one job when ready to send. Attempts fences off expired workers.
pub async fn claim(pool: &DbPool) -> Result<Option<NotificationJob>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let job = sqlx::query_as::<_,NotificationJob>(
        "UPDATE notification_jobs SET status='processing',attempts=attempts+1,lease_until=now()+interval '2 minutes' WHERE id=(SELECT id FROM notification_jobs WHERE (status IN ('pending','retry') AND available_at<=now()) OR (status='processing' AND lease_until<now()) ORDER BY available_at,id FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING *"
    ).fetch_optional(&mut *tx).await?;
    if let Some(job) = &job {
        sqlx::query("UPDATE notification_logs SET status='processing' WHERE id=$1")
            .bind(job.notification_log_id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(job)
}

pub async fn allowed(pool: &DbPool, job: &NotificationJob) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM notification_logs l JOIN notification_settings s ON s.notification_event_id=l.notification_event_id WHERE l.id=$1 AND s.enabled AND s.email_enabled AND EXISTS(SELECT 1 FROM notification_recipients r WHERE r.notification_event_id=l.notification_event_id AND r.enabled AND lower(btrim(r.email))=lower(btrim($2))))")
        .bind(job.notification_log_id).bind(&job.recipient_email).fetch_one(pool).await
}

/// Commit the job and its user-visible log outcome atomically.
pub async fn finish(
    pool: &DbPool,
    job: &NotificationJob,
    status: &str,
    error: Option<&str>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let changed=sqlx::query("UPDATE notification_jobs SET status=$3,error_message=$4,processed_at=CASE WHEN $3='retry' THEN NULL ELSE now() END,lease_until=NULL,available_at=now()+make_interval(secs=>least(3600,30*power(2,least(attempts,7)))::double precision) WHERE id=$1 AND attempts=$2 AND status='processing'")
        .bind(job.id).bind(job.attempts).bind(status).bind(error).execute(&mut *tx).await?;
    if changed.rows_affected() == 1 {
        sqlx::query("UPDATE notification_logs SET status=$2,error_message=$3,sent_at=CASE WHEN $2='sent' THEN now() ELSE sent_at END WHERE id=$1")
            .bind(job.notification_log_id).bind(status).bind(error).execute(&mut *tx).await?;
    }
    tx.commit().await
}

pub async fn process_pending_jobs(pool: &DbPool, config: &Config) -> Result<(), sqlx::Error> {
    process_with_sender(pool, config, |message| email_sender::send(config, message)).await
}

pub async fn process_with_sender<F, Fut>(
    pool: &DbPool,
    config: &Config,
    send: F,
) -> Result<(), sqlx::Error>
where
    F: Fn(EmailMessage) -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    // An outbox/rendering failure must not stall already queued WSO/PO mail.
    let enqueue_error = po_notifications::enqueue_occurrences(pool, config)
        .await
        .err();
    for _ in 0..100 {
        let Some(job) = claim(pool).await? else {
            break;
        };
        if !allowed(pool, &job).await? {
            finish(
                pool,
                &job,
                "suppressed",
                Some("Email event or recipient disabled before delivery"),
            )
            .await?;
            continue;
        }
        if job.attempts > 5 {
            finish(
                pool,
                &job,
                "failed",
                Some("Delivery attempt limit reached after an expired lease"),
            )
            .await?;
            continue;
        }
        let message = EmailMessage {
            from_name: job.sender_name.clone(),
            from_email: job.sender_email.clone(),
            to: job.recipient_email.clone(),
            subject: job.subject.clone(),
            html_body: job.html_body.clone(),
        };
        // Bound SMTP below the lease. Acceptance followed by a crash can still
        // duplicate an email: SMTP delivery is at-least-once, not exactly-once.
        match tokio::time::timeout(std::time::Duration::from_secs(60), send(message)).await {
            Ok(Ok(())) => finish(pool, &job, "sent", None).await?,
            result => {
                let error = match result {
                    Ok(Err(e)) => e,
                    _ => "SMTP operation timed out".into(),
                };
                let permanent = error.starts_with("Invalid ")
                    || error.starts_with("Failed to build email")
                    || error.starts_with("Permanent SMTP failure");
                finish(
                    pool,
                    &job,
                    if permanent || job.attempts >= 5 {
                        "failed"
                    } else {
                        "retry"
                    },
                    Some(&error),
                )
                .await?;
            }
        }
    }
    match enqueue_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
