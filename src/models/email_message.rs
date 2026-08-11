#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from_name: String,
    pub from_email: String,

    pub to: String,
    pub subject: String,
    pub html_body: String,
}