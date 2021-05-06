use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_success() {
    let srv = given("record/smoke/success");
    isahc::get(srv.path("/success")).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/success")).unwrap().assert_ok();
    assert_recorded_stub_eq("success-3335369288306863837", json!({
        "request": {
            "method": "GET",
            "urlPath": "/success"
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_errors() {
    let srv = given("record/smoke/success");
    isahc::get(srv.path("/not-found")).unwrap().assert_not_found();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/not-found")).unwrap().assert_not_found();
    assert_recorded_stub_eq("not-found-2690652350161762789", json!({
        "request": {
            "method": "GET",
            "urlPath": "/not-found"
        },
        "response": {"status": 404}
    }))
}