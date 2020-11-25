#![feature(proc_macro_hygiene, decl_macro)]
mod models;
mod tests;

use rocket::{post, routes};
use serde_json::json;

#[post("/mutate")]
fn mutater() -> String {
    let response = json!({"foo": "bar"});
    response.to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![mutater]).launch();
}