use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_status_200() {
    let srv = given("record/status/200");
    isahc::get(srv.path("/status/200")).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/status/200")).unwrap().assert_ok();
    assert_recorded_stub_eq("status-200-1330526116653087821", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/200"
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_status_400() {
    let srv = given("record/status/400");
    isahc::get(srv.path("/status/400")).unwrap().assert_bad_request();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/status/400")).unwrap().assert_bad_request();
    assert_recorded_stub_eq("status-400-13127736630424190359", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/400"
        },
        "response": {"status": 400}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_status_500() {
    let srv = given("record/status/500");
    isahc::get(srv.path("/status/500")).unwrap().assert_error();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/status/500")).unwrap().assert_error();
    assert_recorded_stub_eq("status-500-13973602113803580223", json!({
        "request": {
            "method": "GET",
            "urlPath": "/status/500"
        },
        "response": {"status": 500}
    }))
}
