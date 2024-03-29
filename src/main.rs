use dbofdb::{self, prelude::MainTable};
use log::{trace,debug};
use serde_json::json;
use warp::Filter;
#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();
    let conn = dbofdb::establish_connection();
    let value = json!({
    "name": "World",
    "cost": 3000,
    "val":0,
    "time":"2020-11-11 00:00:00"
    });

    // Generating Schema on the Fly from Jsonb Data
    // Only Testing
    let schema_from_json_value = jsonschema::JSONSchema::compile(&value).unwrap();

    trace!("{:#?}", schema_from_json_value);
    trace!("{:#?}", schema_from_json_value.is_valid(&value));

    // Get the Schema for the Data. Yet to get the Schema from the json Object.
    // Only Testing
    let schema = schemars::schema_for!(dbofdb::Data);

    trace!("{}", &serde_json::to_string_pretty(&schema).unwrap());

    // Getting Time Ranged Data Works.
    let r = MainTable::query_by_interval(&conn, chrono::Utc::now(), chrono::Utc::now()).unwrap(); // Probably use `thiserror` Lib for Error Handling.

    debug!("Main table Return Data {:?}",r);

    // Small rest Api to test.
    let insert_data = warp::post()
        .and(warp::path("data"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|data: dbofdb::Data| warp::reply::json(&data));

    warp::serve(insert_data).run(([127, 0, 0, 1], 3030)).await;
}
