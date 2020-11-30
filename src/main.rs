use dbofdb;

extern crate diesel;
use dbofdb::models::*;
use dbofdb::schema::main_table;
use diesel::{pg::Pg, prelude::*};

fn main() {
    let conn = dbofdb::establish_connection();
    let maintable_data = InsertIntoMainTable::default();
    let r: MainTable = diesel::insert_into(main_table::table)
        .values(&maintable_data)
        .get_result(&conn)
        .expect("Failed to add anything");
}
