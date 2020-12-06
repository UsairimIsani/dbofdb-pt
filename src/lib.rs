#[macro_use]
extern crate diesel;

mod controllers;
mod models;
mod routes;
mod schema;

mod items;
mod operation;
mod setting;

pub mod data_form;
pub use data_form::Data;
pub use items::Items;
pub use operation::Operation;
pub use setting::Setting;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
pub mod prelude {
    pub use super::*;
    // pub use Data;
}
pub fn establish_connection() -> PgConnection {
    log::info!("Establlishing Connection to Timescale DB");
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
            "command": "delete",
            "items": [
                {
                    "tags": {},
                    "data": {},
                    "entity": "price",
                    "time": "2020-11-11 00:00:00"
                },
                {
                    "tags": {},
                    "data": 
                    {
                        "name": "World",
                        "cost": 3000000000
                    },
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
