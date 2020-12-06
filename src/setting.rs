use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Setting {
    aggregates: Value,
}
