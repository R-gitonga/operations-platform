#[derive(Debug, Clone)]
pub struct EmailMessage {

    pub to: String,

    pub subject: String,

    pub html_body: String,
}