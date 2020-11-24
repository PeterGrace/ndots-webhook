use std::fs;
use serde_json;
use crate::models::AdmissionReview;
#[test]
fn test_noop () -> () {}


#[test]
fn deserialize_review() {
    let review_json = fs::read_to_string("./src/tests/admission-review.json").expect("Unable to read file!");
    let review: AdmissionReview = serde_json::from_str(review_json.as_str()).unwrap();
    assert_eq!(review.)
}