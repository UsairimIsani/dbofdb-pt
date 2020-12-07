use crate::schema::main_table;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use log::trace;
use serde_json::value::Value;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "main_table"]
#[primary_key(insert_time)]
pub struct MainTable {
    pub insert_time: DateTime<Utc>,
    pub reference_time: DateTime<Utc>,
    pub data: Value,
    pub tags: Value,
    pub bucket_name: String,
}

type AllColumns = (main_table::data, main_table::tags);

type All = diesel::dsl::Select<main_table::table, AllColumns>;

type WithInterval<'a> = diesel::dsl::Between<main_table::insert_time, DateTime<Utc>, DateTime<Utc>>;

type ByInterval<'a> = diesel::dsl::Filter<All, WithInterval<'a>>;

pub const ALL_COLUMNS: AllColumns = (main_table::data, main_table::tags);

impl MainTable {
    pub fn get_all() -> All {
        main_table::table.select(ALL_COLUMNS)
    }

    // Need to figure Out Time interval Types
    pub fn query_by_interval(
        conn: &PgConnection,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<(Value,Value)>> {
        let r = main_table::table
            .select(ALL_COLUMNS)
            .filter(main_table::insert_time.between(start, end)) // ByInterval Type Constructed Here
            .load::<(Value,Value)>(conn)?;
        trace!("Get Time ranged Data. "); 
        Ok(r)
    }
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

