use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/req-body/json.json")]
async fn proxy_should_forward_json_request_body() {
    isahc::post(stubr.path("/body/req/json"), json!({"a": { "b": "c" }}).to_string()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .post(stubr.path("/body/req/json"), json!({"a": { "b": "c" }}).to_string())
        .expect_status_ok();
    assert_recorded_stub_eq("body-req-json-1875300994429885227", json!({
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