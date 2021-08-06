use reqwest::blocking::ClientBuilder as ReqwestBlockingClientBuilder;
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
    ReqwestBlockingClientBuilder::new().build().unwrap()
        .get(stubr.path("/record-client/reqwest"))
        .record_with(cfg);
    assert_recorded_stub_eq("record-client-reqwest-1024385884503042741", json!(
                                    {
                                        "request": {
                                            "method": "GET",
                                            "urlPath": "/record-client/reqwest"
                                        },
                                        "response": {
                                            "status": 200
                                        }
                                    }
    ))
}


#[test]
#[stubr::mock("record/record-client")]
fn should_record_from_reqwest_client_ko() {
    let cfg = RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    ReqwestBlockingClientBuilder::new().build().unwrap()
        .get(stubr.path("/record-client/reqwest/ko"))
        .record_with(cfg);
    assert_recorded_stub_eq("record-client-reqwest-ko-3875024637254819038", json!(
                                    {
                                        "request": {
                                            "method": "GET",
                                            "urlPath": "/record-client/reqwest/ko"
                                        },
                                        "response": {
                                            "status": 404
                                        }
                                    }
    ))
}