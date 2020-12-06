use crate::{models::InsertIntoMainTable, schema::main_table::dsl::*};
use diesel::prelude::*;
use diesel::{dsl::insert_into, PgConnection};

pub fn insert_vals_into_main_table(
    conn: &PgConnection,
    records: Vec<InsertIntoMainTable>,
) -> anyhow::Result<()> {
    insert_into(main_table).values(records).execute(conn)?;
    Ok(())
}
