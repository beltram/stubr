use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/smoke/success.json")]
async fn proxy_should_forward_success() {
    isahc::get(stubr.path("/success")).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/success")).expect_status_ok();
    assert_recorded_stub_eq("success-3335369288306863837", json!({
        "request": {
            "method": "GET",
            "urlPath": "/success"
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/smoke/success.json")]
async fn proxy_should_forward_errors() {
    isahc::get(stubr.path("/not-found")).expect_status_not_found();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/not-found")).expect_status_not_found();
    assert_recorded_stub_eq("not-found-2690652350161762789", json!({
        "request": {
            "method": "GET",
            "urlPath": "/not-found"
        },
        "response": {"status": 404}
    }))
}