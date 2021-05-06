use isahc::Request;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_request_headers() {
    let srv = given("record/req-headers/one");
    isahc::send(req_header(srv.path("/headers/req/one"), &[("x-a", "a")])).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .send(req_header(srv.path("/headers/req/one"), &[("x-a", "a")]))
        .unwrap().assert_ok();
    assert_recorded_stub_eq("headers-req-one-4429402255848487673", json!({
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
async fn proxy_should_forward_many_request_headers() {
    let srv = given("record/req-headers/many");
    isahc::send(req_header(srv.path("/headers/req/many"), &[("x-a", "a"), ("x-b", "b")])).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .send(req_header(srv.path("/headers/req/many"), &[("x-a", "a"), ("x-b", "b")]))
        .unwrap().assert_ok();
    assert_recorded_stub_eq("headers-req-many-7310784668229424867", json!({
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
