use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdatePoSettings {
    pub overdue_grace_days: i32,
    pub approaching_window_days: i32,
    pub stalled_after_days: i32,
    pub unresolved_defect_after_days: i32,
    pub alert_sweep_interval_minutes: i32,
}