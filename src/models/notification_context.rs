use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NotificationContext {
    pub event_code: String,

    pub actor_name: String,

    pub actor_email: String,

    pub variables: HashMap<String, String>,
}

impl NotificationContext {

    pub fn get(&self, key: &str) -> String {

        self.variables
            .get(key)
            .cloned()
            .unwrap_or_else(|| "-".to_string())
    }

    pub fn contains(&self, key: &str) -> bool {

        self.variables.contains_key(key)
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {

        self.variables.insert(
            key.into(),
            value.into(),
        );
    }
}