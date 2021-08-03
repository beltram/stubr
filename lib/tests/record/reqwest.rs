use asserhttp::*;
use reqwest::blocking::ClientBuilder as ReqwestBlockingClientBuilder;
use serde_json::json;

use stubr::Record;

use crate::utils::*;

#[test]
#[stubr::mock("record/record-client/reqwest.json")]
fn should_record_from_reqwest_client() {
    ReqwestBlockingClientBuilder::new().build().unwrap()
        .get(stubr.path("/record-client/reqwest"))
        .record();
    assert_recorded_stub_eq("record-client-reqwest-16401439830736972376", json!({
        "request": {"method": "GET"},
        "response": {"status": 200}
    }))
}