
//to read later
/// Rust Traits
/// Clone Trait
/// Ownership 

use crate::database::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}


