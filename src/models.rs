use diesel::sql_types::*;
use super::schema::*;
use serde_derive::*;

#[derive(Queryable)]
pub struct Point {
    pub id: i32,
    pub tag: String,
    pub created: i64,
    pub value: f64,
}


#[derive(Insertable, Deserialize)]
#[table_name = "points"]
pub struct NewPoint<'a> {
    pub tag: &'a str,
    pub value: f64,
}