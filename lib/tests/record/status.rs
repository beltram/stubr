use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/200.json")]
async fn proxy_should_forward_status_200() {
    isahc::get(stubr.path("/status/200")).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/status/200")).expect_status_ok();
    assert_recorded_stub_eq("status-200-18234664605884201165", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/200"
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/400.json")]
async fn proxy_should_forward_status_400() {
    isahc::get(stubr.path("/status/400")).expect_status_bad_request();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/status/400")).expect_status_bad_request();
    assert_recorded_stub_eq("status-400-9868264260807328606", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/400"
        },
        "response": {"status": 400}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/500.json")]
async fn proxy_should_forward_status_500() {
    isahc::get(stubr.path("/status/500")).expect_status_internal_server_error();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/status/500")).expect_status_internal_server_error();
    assert_recorded_stub_eq("status-500-7794663214452839295", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/500"
        },
        "response": {"status": 500}
    }))
}
