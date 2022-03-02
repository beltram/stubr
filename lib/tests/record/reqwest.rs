use asserhttp::*;
use reqwest::blocking::{Client, ClientBuilder};
use serde_json::json;

use stubr::{Record, RecordConfig};

use crate::utils::*;

#[test]
#[stubr::mock("record/record-client")]
fn should_record_from_reqwest_client() {
    let cfg = RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    let uri = "/record-client/reqwest";
    let req = ClientBuilder::new().build().unwrap()
        .get(stubr.path(uri))
        .record_with(cfg)
        .build().unwrap();
    Client::default().execute(req).unwrap()
        .expect_status_ok();
    assert_recorded_stub_eq("record-client-reqwest-17934677986753600968", json!({
        "request": {
            "method": "GET",
            "urlPath": uri
        },
        "response": {
            "status": 200
        }
    }))
}


#[test]
#[stubr::mock("record/record-client")]
fn should_record_from_reqwest_client_ko() {
    let cfg = RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    let uri = "/record-client/reqwest/ko";
    let req = ClientBuilder::new().build().unwrap()
        .get(stubr.path(uri))
        .record_with(cfg)
        .build().unwrap();
    Client::default().execute(req).unwrap()
        .expect_status_internal_server_error();
    assert_recorded_stub_eq("record-client-reqwest-ko-15261495417896979192", json!({
        "request": {
            "method": "GET",
            "urlPath": uri
        },
        "response": {
            "status": 500
        }
    }))
}