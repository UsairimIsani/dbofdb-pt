use dbofdb;
use dbofdb::models::*;
use dbofdb::schema::main_table;

use diesel::prelude::*;
use serde_json::{json, Value};

fn main() {
    let conn = dbofdb::establish_connection();
    let default_data = InsertIntoMainTable::default();
    let some_data = InsertIntoMainTable::new(json!(6), Value::Bool(false), String::from("price"));
    let more_data = InsertIntoMainTable::new(
        json!(6),
        json!({"hello":"world","welcome":"dbofdb"}),
        String::from("price"),
    );
    let r: Vec<MainTable> = diesel::insert_into(main_table::table)
        .values(&vec![default_data, some_data, more_data])
        .get_results(&conn)
        .expect("Failed to add anything");
}
