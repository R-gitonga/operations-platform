use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChangeWsoStageRequest {
    pub production_stage_id: i32,

    pub notes: Option<String>,

    pub changed_by: String,
}