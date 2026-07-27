use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedEmail {
    pub subject: String,
    pub html_body: String,
}