use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/resp-headers/one.json")]
async fn proxy_should_forward_response_headers() {
    isahc::get(stubr.path("/headers/resp/one"))
        .expect_status_ok()
        .expect_header("x-a", "a");
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/headers/resp/one"))
        .expect_status_ok()
        .expect_header("x-a", "a");
    assert_recorded_stub_eq("headers-resp-one-15944799029425559929", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/resp/one"
        },
        "response": {
            "status": 200,
            "headers": {
                "x-a": "a"
            }
        }
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/resp-headers/many.json")]
async fn proxy_should_forward_many_response_headers() {
    isahc::get(stubr.path("/headers/resp/many"))
        .expect_status_ok()
        .expect_header("x-a", "a")
        .expect_header("x-b", "b");
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.path("/headers/resp/many"))
        .expect_status_ok()
        .expect_header("x-a", "a")
        .expect_header("x-b", "b");
    assert_recorded_stub_eq("headers-resp-many-5360517033018629606", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/resp/many"
        },
        "response": {
            "status": 200,
            "headers": {
                "x-a": "a",
                "x-b": "b"
            }
        }
    }))
}
