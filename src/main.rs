#![feature(proc_macro_hygiene, decl_macro)]
mod models;
mod tests;

use rocket::{post, routes};
use serde_json::{json, Value};
use rocket_contrib::json::Json;
use crate::models::{AdmissionReview, AdmissionRequest, AdmissionResponse, StatusResult};
use rocket::logger::warn;
use rocket::http::Status;

fn generate_error_response(uid: String, msg: &str) -> Json<AdmissionReview> {
    let message = StatusResult {
        code:Some(400),
        reason:Some(msg.to_string()),
        message:Some(msg.to_string()),
        details:None,
        status:Some(msg.to_string())
    };
    let messages = vec![message];
    let mut response = AdmissionResponse::default();
    response.uid = uid;
    response.allowed = false;
    response.status = Some(messages);
    let mut review = AdmissionReview::default();
    review.response = Some(response);
    Json(review)
}

#[post("/mutate", format = "json", data = "<review>")]
fn mutater(review: Json<AdmissionReview>) -> Json<AdmissionReview> {
    let req = review.request.as_ref().unwrap();
    let obj = req.object.as_ref().unwrap();
    let obj_meta = match obj.get("metadata") {
        Some(o) => o,
        None => return generate_error_response(req.clone().uid, "no metadata in request")
    };
    let mut ndots: Option<String> = None;
    match obj_meta.get("annotations") {
        Some(om) => {
            match om.get("ndots.vsix.me/ndots") {
                Some(num) => {
                    ndots = Some(num.to_string());
                },
                None => {},
            };
        },
        None => {},
    };
    let mut response= AdmissionResponse::default();
    response.uid = req.clone().uid;
    response.allowed = true;
    match ndots {
        Some(ndots) => {
            let patch = format!("[{{\"op\": \"add\", \"path\":\"/spec/dnsConfig/options/-\", \"value\":[{{\"name\":\"ndots\",\"value\":{}}}]}}]",ndots);
            response.patch = Some(base64::encode(patch));
        },
        None => {}
    }
    let mut review = AdmissionReview::default();
    review.response = Some(response);
    Json(review)
}

fn main() {
    rocket::ignite().mount("/", routes![mutater]).launch();
}