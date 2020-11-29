use serde::{Deserialize, Serialize};

use crate::{Items, Operation, Setting};
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    command: Operation, // enum
    items: Vec<Items>,  // Struct
    settings: Setting,  // Struct
}
