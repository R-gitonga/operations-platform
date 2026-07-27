use crate::models::email_message::EmailMessage;

pub async fn send(
    message: EmailMessage,
) -> Result<(), String> {

    println!();
    println!("================ EMAIL =================");

    println!("TO      : {}", message.to);

    println!("SUBJECT : {}", message.subject);

    println!();

    println!("{}", message.html_body);

    println!("========================================");

    Ok(())
}