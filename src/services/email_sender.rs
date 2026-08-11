use lettre::{
    message::{header::ContentType, Mailbox, Message},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Tokio1Executor,
};

use crate::{
    config::Config,
    models::email_message::EmailMessage,
};

pub async fn send(
    config: &Config,
    message: EmailMessage,
) -> Result<(), String> {

    let from: Mailbox = format!(
        "{} <{}>",
        message.from_name,
        message.from_email
    )
    .parse()
    .map_err(|error| {
        format!("Invalid SMTP sender address: {}", error)
    })?;

    let to: Mailbox = message
        .to
        .parse()
        .map_err(|error| {
            format!("Invalid recipient email address: {}", error)
        })?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(message.subject)
        .header(ContentType::TEXT_HTML)
        .body(message.html_body)
        .map_err(|error| {
            format!("Failed to build email: {}", error)
        })?;

    let credentials = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    let mailer =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
            &config.smtp_host,
        )
        .map_err(|error| {
            format!("Failed to configure SMTP transport: {}", error)
        })?
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|error| {
            format!("Failed to send email: {}", error)
        })?;

    Ok(())
}