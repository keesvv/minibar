use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = beverage)]
#[diesel(check_for_backend(Sqlite))]
pub struct Beverage {
    pub id: String,
    pub description: String,
    pub capacity: f32,
    pub amount: f32,
}
