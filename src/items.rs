// use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize)]
pub struct Items {
    tags: Value,
    data: Value,
    entity: String,
    time: String,
}
