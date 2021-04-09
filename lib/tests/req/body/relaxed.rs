use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_match_req_body_equal_to_json_ignoring_array_order_and_extra_elements() {
    let srv = given("req/body/eq/ignore/all-relaxed");
    post(&srv.url()).body(json!({"names": ["john", "doe"]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"names": ["doe", "john"]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"names": ["john", "doe"], "age": 42})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"names": ["doe", "john"], "age": 42})).await.unwrap().assert_ok();
}