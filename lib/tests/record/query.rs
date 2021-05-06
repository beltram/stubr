use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_query_param() {
    let srv = given("record/query/one");
    isahc::get(srv.path_query("/query/one", "a", "1")).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .get(srv.path_query("/query/one", "a", "1"))
        .unwrap().assert_ok();
    assert_recorded_stub_eq("query-one-12579359212080673625", json!({
        "request": {
            "method": "GET",
            "urlPath": "/query/one",
            "queryParameters": {
                "a": { "equalTo": "1" }
            }
        },
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_many_query_param() {
    let srv = given("record/query/many");
    isahc::get(srv.path_queries("/query/many", ("a", "1"), ("b", "2"))).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .get(srv.path_queries("/query/many", ("a", "1"), ("b", "2")))
        .unwrap().assert_ok();
    assert_recorded_stub_eq("query-many-17502612938178303204", json!({
        "request": {
            "method": "GET",
            "urlPath": "/query/many",
            "queryParameters": {
                "a": { "equalTo": "1" },
                "b": { "equalTo": "2" },
            }
        },
        "response": {"status": 200}
    }))
}