use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,

    pub name: String,

    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub role: String,

    pub active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}