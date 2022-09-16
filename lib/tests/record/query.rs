use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/query/one.json")]
async fn proxy_should_forward_query_param() {
    isahc::get(stubr.path_query("/query/one", "a", "1")).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .get(stubr.path_query("/query/one", "a", "1"))
        .expect_status_ok();
    assert_recorded_stub_eq("query-one-14477591495544065577", json!({
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
#[stubr::mock("record/query/many.json")]
async fn proxy_should_forward_many_query_param() {
    isahc::get(stubr.path_queries("/query/many", ("a", "1"), ("b", "2"))).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .get(stubr.path_queries("/query/many", ("a", "1"), ("b", "2")))
        .expect_status_ok();
    assert_recorded_stub_eq("query-many-15093486744474350709", json!({
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