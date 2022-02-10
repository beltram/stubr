use asserhttp::*;
use isahc::Request;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/req-headers/one.json")]
async fn proxy_should_forward_request_headers() {
    isahc::send(req_header(stubr.path("/headers/req/one"), &[("x-a", "a")])).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .send(req_header(stubr.path("/headers/req/one"), &[("x-a", "a")]))
        .expect_status_ok();
    assert_recorded_stub_eq("headers-req-one-3603143203592242792", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/req/one",
            "headers": {
                "x-a": { "equalTo": "a" }
            }
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/req-headers/many.json")]
async fn proxy_should_forward_many_request_headers() {
    isahc::send(req_header(stubr.path("/headers/req/many"), &[("x-a", "a"), ("x-b", "b")])).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .send(req_header(stubr.path("/headers/req/many"), &[("x-a", "a"), ("x-b", "b")]))
        .expect_status_ok();
    assert_recorded_stub_eq("headers-req-many-2405737596588286623", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/req/many",
            "headers": {
                "x-a": { "equalTo": "a" },
                "x-b": { "equalTo": "b" }
            }
        },
        "response": {"status": 200}
    }))
}

fn req_header(uri: String, values: &[(&str, &str)]) -> Request<()> {
    let mut base = Request::get(uri);
    for (k, v) in values {
        base = base.header(*k, *v);
    }
    base.body(()).unwrap()
}
