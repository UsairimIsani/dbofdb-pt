// use chrono::{DateTime, Utc};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Items {
    tags: Value,
    data: Value,
    entity: String,
    time: String,
}
