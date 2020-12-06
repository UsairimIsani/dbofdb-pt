use crate::schema::main_table;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde_json::value::Value;

#[derive(Queryable, Identifiable, Associations)]
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
    ) -> anyhow::Result<()> {
        let a = main_table::table
            .select(ALL_COLUMNS)
            .filter(main_table::insert_time.between(start, end)) // ByInterval Type Constructed Here
            .execute(conn)?;
        log::debug!("Get Timeranged Data");
        println!("Get Timeranged  {:?}", a);
        Ok(())
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
// use diesel::types::{Date, Text};
// sql_function!(canon_crate_name, canon_crate_name_t, (x: Text) -> Text);
// use diesel::pg::Pg;
// use diesel::prelude::*;
// use diesel::query_builder::*;
// use diesel::query_dsl::methods::LoadQuery;
// use diesel::sql_types::BigInt;

// pub trait Intervaled: Sized {
//     fn interval(self, start: DateTime<Utc>, end: DateTime<Utc>) -> Interval<Self>;
// }

// impl<T> Intervaled for T {
//     fn interval(self, start: DateTime<Utc>, end: DateTime<Utc>) -> Interval<Self> {
//         Interval {
//             query: self,
//             start,
//             end,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, QueryId)]
// pub struct Interval<T> {
//     query: T,
//     start: DateTime<Utc>,
//     end: DateTime<Utc>,
// }

// impl<T> Interval<T> {}

// // impl<T: Query> Query for Interval<T> {
// //     type SqlType = (T::SqlType, BigInt);
// // }

// impl<T> RunQueryDsl<PgConnection> for Interval<T> {}

// impl<T> QueryFragment<Pg> for Interval<T>
// where
//     T: QueryFragment<Pg>,
// {
//     //   SELECT time_bucket('15 minutes', time) AS fifteen_min,
//     //     data,
//     //   FROM main_table
//     //   WHERE time > NOW() - INTERVAL '3 hours' // Map to Duration(chrono or Rust).
//     //   GROUP BY fifteen_min, data
//     //   ORDER BY fifteen_min DESC;
//     fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
//         out.push_sql("SELECT * FROM");
//         self.query.walk_ast(out.reborrow())?;
//         out.push_sql("");
//         out.push_sql("BETWEEN");
//         out.push_bind_param::<diesel::sql_types::Timestamptz, _>(&self.start)?;
//         out.push_sql("AND");
//         out.push_bind_param::<diesel::sql_types::Timestamptz, _>(&self.start)?;
//         Ok(())
//     }
// }
