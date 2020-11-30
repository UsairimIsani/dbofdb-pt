use crate::schema::main_table;

use chrono::{DateTime, Utc};
use diesel::{expression::subselect::ValidSubselect, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Queryable)]
pub struct MainTable {
    pub id: i32,
    pub reference_time: DateTime<Utc>,
    pub insert_time: DateTime<Utc>,
    pub data: Value,
    pub tags: Value,
    pub bucket_name: String,
}
#[derive(Insertable)]
#[table_name = "main_table"]
pub struct InsertIntoMainTable {
    pub reference_time: DateTime<Utc>,
    pub insert_time: DateTime<Utc>,
    pub data: Value,
    pub tags: Value,
    pub bucket_name: String,
}
impl InsertIntoMainTable {
    pub fn new(data: Value, tags: Value, bucket_name: String) -> Self {
        Self {
            data,
            tags,
            bucket_name,
            reference_time: Utc::now(),
            insert_time: Utc::now(),
        }
    }
}
impl Default for InsertIntoMainTable {
    fn default() -> Self {
        Self {
            reference_time: Utc::now(),
            insert_time: Utc::now(),
            data: Value::Null,
            tags: Value::Null,
            bucket_name: String::new(),
        }
    }
}
