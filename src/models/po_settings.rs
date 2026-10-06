use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PoSettings {
    pub id: i32,
    pub overdue_grace_days: i32,
    pub approaching_window_days: i32,
    pub stalled_after_days: i32,
    pub unresolved_defect_after_days: i32,
    pub alert_sweep_interval_minutes: i32,
    pub updated_at: DateTime<Utc>,
}