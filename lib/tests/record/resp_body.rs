use asserhttp::*;
use serde_json::json;

use stubr::{RecordConfig, Stubr};

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/resp-body/json.json")]
async fn proxy_should_forward_json_response_body() {
    isahc::get(stubr.path("/body/resp/json"))
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({"a": {"b": "c"}}));
    Stubr::record_with(resp_body_cfg()).isahc_client().get(stubr.path("/body/resp/json"))
        .expect_status_ok()
        .expect_content_type_json()
        .expect_body_json_eq(json!({"a": {"b": "c"}}));
    assert_recorded_stub_eq("body-resp-json-4281328719312332612", json!({
        "request": {
            "method": "GET",
            "urlPath": "/body/resp/json"
        },
        "response": {
            "status": 200,
            "headers": { "content-type": "application/json" },
            "jsonBody": {"a": { "b": "c" }}
        }
    }))
}

fn resp_body_cfg() -> RecordConfig {
    RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(resp_headers_with_content_type()),
        ..Default::default()
    }
}