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
use dotenv::dotenv;
use reqwest;

//fn threshold() {
//    dotenv("THRESHOLD").map(|t| t.parse() as i64).or_else(1)
//}

#[post("/api/point", format = "application/json", data = "<point>")]
fn add_point(point: Json<NewPoint>, connection: DbConn) -> String {
    let slack_url = dotenv::var("SLACK_URL").expect("No slack url configured :(");
    let threshold: f64 = dotenv::var("THRESHOLD").expect("No threshold configured :(").parse().unwrap();
    if &point.value > &threshold {
        let mut map = HashMap::new();
        map.insert("text", "COFFEE");
        let client = reqwest::Client::new();
        let res = client.post(&slack_url)
            .json(&map)
            .send();
    }
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

/// Configure Rocket to serve on the port requested by Heroku.
fn configure() -> rocket::Config {
    let mut config = rocket::Config::active().expect("could not load configuration");
    if let Ok(port_str) = dotenv::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

fn main() {
    rocket::custom(configure())
        .manage(connection::init_pool())
        .mount("/", routes![index, add_point])
        .attach(Template::fairing())
        .launch();
}