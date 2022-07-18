use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Case {
    pub description: String,
    pub schema: serde_json::Value,
    pub tests: Vec<Test>,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub description: String,
    pub instance: serde_json::Value,
    pub valid: Option<bool>,
}
