use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_json_request_body() {
    let srv = given("record/req-body/json");
    isahc::post(srv.path("/body/req/json"), json!({"a": { "b": "c" }}).to_string()).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .post(srv.path("/body/req/json"), json!({"a": { "b": "c" }}).to_string())
        .unwrap().assert_ok();
    assert_recorded_stub_eq("body-req-json-9547837113797218183", json!({
        "request": {
            "method": "POST",
            "urlPath": "/body/req/json",
            "bodyPatterns": [
                {"equalToJson": {"a": {"b": "c"}}}
            ]
        },
        "response": {"status": 200}
    }))
}