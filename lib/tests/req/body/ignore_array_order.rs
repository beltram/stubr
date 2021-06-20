use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/on.json")]
async fn should_match_req_body_equal_to_json_ignoring_array_order() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/on.json")]
async fn should_not_match_req_body_equal_to_json_ignoring_array_order_when_key_mismatches() {
    post(stubr.uri()).body(json!({"not-names": ["john", "doe"]})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/on.json")]
async fn should_not_match_req_body_equal_to_json_ignoring_array_order_when_items_invalid() {
    post(stubr.uri()).body(json!({"names": ["john"]})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"names": ["doe"]})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"names": []})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"names": ["john", "doe", "alfred"]})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/off.json")]
async fn should_match_req_body_equal_to_json_not_ignoring_array_order() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/default.json")]
async fn by_default_should_match_req_body_equal_to_json_not_ignoring_array_order() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(stubr.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/on.json")]
async fn ignoring_array_order_should_not_allow_extra_elements() {
    post(stubr.uri()).body(json!({"names": ["john", "doe"], "age": 42})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/order/root-array.json")]
async fn should_match_req_body_equal_to_json_ignoring_array_order_for_root_array() {
    post(stubr.uri()).body(json!(["john", "doe"])).await.expect_status_ok();
    post(stubr.uri()).body(json!(["doe", "john"])).await.expect_status_ok();
}
