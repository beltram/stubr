use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/all-relaxed.json")]
async fn should_match_req_body_equal_to_json_ignoring_array_order_and_extra_elements() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["john", "doe"], "age": 42})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["doe", "john"], "age": 42})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/none.json")]
async fn should_match_req_body_equal_to_json_not_ignoring_array_order_nor_extra_elements() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"names": ["john", "doe"], "age": 42})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"names": ["doe", "john"], "age": 42})).await.expect_status_not_found();
}