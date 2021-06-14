use asserhttp::*;
use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_match_req_body_equal_to_json_ignoring_array_order_and_extra_elements() {
    let srv = given("req/body/eq/ignore/all-relaxed");
    post(&srv.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(&srv.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_ok();
    post(&srv.uri()).body(json!({"names": ["john", "doe"], "age": 42})).await.expect_status_ok();
    post(&srv.uri()).body(json!({"names": ["doe", "john"], "age": 42})).await.expect_status_ok();
}