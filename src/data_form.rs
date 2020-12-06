use serde::{Deserialize, Serialize};

use crate::{Items, Operation, Setting};
use schemars::JsonSchema;
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Data {
    command: Operation, // enum
    items: Vec<Items>,  // Struct
    settings: Setting,  // Struct
}
