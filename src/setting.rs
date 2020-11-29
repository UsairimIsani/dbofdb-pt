use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    aggregates: Value,
}
