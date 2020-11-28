#![feature(proc_macro_hygiene, decl_macro)]

mod models;
mod tests;

use crate::models::{AdmissionRequest, AdmissionResponse, AdmissionReview, StatusResult};
use anyhow::{anyhow, Result};
use log::info;
use rocket::http::Status;
use rocket::logger::warn;
use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde_json::{json, Map, Value};
use std::error;
use std::error::Error;

fn generate_error_response(uid: String, msg: &str) -> Json<AdmissionReview> {
    let message = StatusResult {
        code: Some(400),
        reason: Some(msg.to_string()),
        message: Some(msg.to_string()),
        details: None,
        status: Some(msg.to_string()),
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

fn generate_ndots_patch(object: Map<String, Value>, ndots: String) -> Result<String> {
    let mut patches = Vec::new();
    let spec = match object.get("spec") {
        Some(s) => s,
        None => return Err(anyhow!("no spec?")),
    };
    let dns_config = match spec.get("dnsConfig") {
        Some(d) => d,
        None => {
            info!("There was no preexisting dnsConfig, generating a tree.");
            patches.push(json! ({
            "op":"add",
            "path":"/spec/",
            "value":{
                "dnsConfig":{}
                }
            }));
            patches.push(json! ({
                "op":"add",
                "path":"/spec/dnsConfig",
                "value":{"options":[]
                }
            }));
            patches.push(json! ({
                "op":"add",
                "path":"/spec/dnsConfig/options/-",
                "value":{
                    "name": "ndots",
                    "value": ndots.parse().unwrap_or(1)
                }
            }));
            return Ok(json!(patches).to_string());
        }
    };
    let options = match dns_config.get("options") {
        Some(o) => {
            info!("Options exist!  Pushing in our updated ndots.");
            patches.push(json!( {
             "op":"add",
            "path":"/spec/dnsConfig/options/-",
            "value":{
                "name": "ndots",
                "value": ndots
            }
            }));
        }
        None => {
            info!("dnsConfig existed, but no options, so adding that before ndots.");
            patches.push(json! ({
                "op":"add",
                "path":"/spec/dnsConfig",
                "value":{"options":[]
                }
            }));
            patches.push(json! ({
                "op":"add",
                "path":"/spec/dnsConfig/options/-",
                "value":{
                    "name": "ndots",
                    "value": ndots
                }
            }));
        }
    };
    Ok(json!(patches).to_string())
}

#[post("/mutate", format = "json", data = "<review>")]
fn mutater(review: Json<AdmissionReview>) -> Json<AdmissionReview> {
    let req = review.request.as_ref().unwrap();
    let obj = req.object.as_ref().unwrap();
    let obj_meta = match obj.get("metadata") {
        Some(o) => o,
        None => return generate_error_response(req.clone().uid, "no metadata in request"),
    };
    let mut ndots: Option<String> = None;
    match obj_meta.get("annotations") {
        Some(om) => {
            info!("got annotations");
            match om.get("ndots.vsix.me/ndots") {
                Some(num) => {
                    info!("setting ndots to {}", num);
                    ndots = Some(num.to_string());
                }
                None => {
                    info!("No ndots.vsix.me/ndots annotation found");
                }
            };
        }
        None => {
            info!("This podspec didn't have any annotations to investigate.");
        }
    };

    let mut response = AdmissionResponse::default();
    response.uid = req.clone().uid;
    response.allowed = true;
    match ndots {
        Some(ndots) => {
            let patch = match generate_ndots_patch(obj.clone(), ndots) {
                Ok(p) => p,
                Err(e) => {
                    return generate_error_response(req.clone().uid, e.to_string().as_str());
                }
            };
            response.patch = Some(base64::encode(patch));
        }
        None => {}
    }
    let mut review = AdmissionReview::default();
    review.response = Some(response);
    info!("Returning a review object that should be well-formed");
    Json(review)
}

fn main() {
    rocket::ignite().mount("/", routes![mutater]).launch();
}
