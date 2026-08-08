use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChangeProductionItemStageRequest {
    pub production_stage_id: i32,

    pub notes: Option<String>,
}
