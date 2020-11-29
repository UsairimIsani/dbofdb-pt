#[macro_use]

mod models;
mod schema;

pub mod data_form;
pub mod items;
pub mod operation;
pub mod setting;

pub use data_form::Data;
pub use items::Items;
pub use operation::Operation;
pub use setting::Setting;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

mod tests {
    #[test]
    fn parse_json_to_data() {
        use crate::Data;
        let input = r#"
        {
            "command": "insert",
            "items": [
                {
                    "tags": {},
                    "data": {},
                    "entity": "price",
                    "time": "2020-11-11 00:00:00"
                }
            ],
            "settings": {
                "aggregates": {}
            }
        }
        "#;
        let data: Data = serde_json::from_str(input).unwrap();
        println!("{:#?}", data);
    }
}
