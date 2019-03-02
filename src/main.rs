#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use spylent::models::*;
use spylent::connection;
use diesel;
use spylent::schema::*;
use spylent::connection::*;
use diesel::query_dsl::*;
use diesel::QueryResult;


#[post("/api/point", format = "application/json", data = "<point>")]
fn add_point(point: Json<NewPoint>, connection: DbConn) -> String {
    let new_point: NewPoint = point.into_inner();
    let _: QueryResult<Point> = diesel::insert_into(points::table)
        .values(&new_point)
        .get_result(&*connection);
    "Success".into()
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![index, add_point])
        .attach(Template::fairing())
        .launch();
}