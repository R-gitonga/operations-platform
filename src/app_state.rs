use crate::{
    config::Config,
    database::DbPool,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}