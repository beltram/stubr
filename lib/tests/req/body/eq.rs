use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("req/body/eq/string.json")]
async fn should_map_req_body_equal_to_json_string() {
    post(stubr.uri()).body(json!({"name": "bob"})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/string.json")]
async fn should_fail_when_req_body_key_not_equal_to_json_string() {
    post(stubr.uri())
        .body(json!({"notName": "bob"}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/string.json")]
async fn should_fail_when_req_body_value_not_equal_to_json_string() {
    post(stubr.uri())
        .body(json!({"name": "not-bob"}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/bool.json")]
async fn should_map_req_body_equal_to_json_bool() {
    post(stubr.uri()).body(json!({"isGood": true})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/bool.json")]
async fn should_fail_when_req_body_key_not_equal_to_json_bool() {
    post(stubr.uri())
        .body(json!({"isNotGood": true}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/bool.json")]
async fn should_fail_when_req_body_value_not_equal_to_json_bool() {
    post(stubr.uri()).body(json!({"isGood": false})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/unsigned-number.json")]
async fn should_map_req_body_equal_to_json_unsigned_number() {
    post(stubr.uri()).body(json!({"age": 42})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/unsigned-number.json")]
async fn should_fail_when_req_body_key_not_equal_to_json_unsigned_number() {
    post(stubr.uri()).body(json!({"notAge": 42})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/unsigned-number.json")]
async fn should_fail_when_req_body_value_not_equal_to_json_unsigned_number() {
    post(stubr.uri()).body(json!({"age": 43})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/signed-number.json")]
async fn should_map_req_body_equal_to_json_signed_number() {
    post(stubr.uri()).body(json!({"age": -42})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/signed-number.json")]
async fn should_fail_when_req_body_key_not_equal_to_json_signed_number() {
    post(stubr.uri()).body(json!({"notAge": -42})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/signed-number.json")]
async fn should_fail_when_req_body_value_not_equal_to_json_signed_number() {
    post(stubr.uri()).body(json!({"age": -43})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/float.json")]
async fn should_map_req_body_equal_to_float() {
    post(stubr.uri()).body(json!({"pi": 14.3})).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/float.json")]
async fn should_fail_when_req_body_key_not_equal_to_float() {
    post(stubr.uri()).body(json!({"notPi": 14.3})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/float.json")]
async fn should_fail_when_req_body_value_not_equal_to_float() {
    post(stubr.uri()).body(json!({"pi": 14.4})).await.expect_status_not_found();
    post(stubr.uri()).body(json!({"pi": 14.2})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/null.json")]
async fn should_map_req_body_equal_to_null() {
    post(stubr.uri()).body(json!({ "maybe": null })).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/null.json")]
async fn should_fail_when_req_body_key_not_equal_to_null() {
    post(stubr.uri()).body(json!({ "sure": null })).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/null.json")]
async fn should_fail_when_req_body_value_not_equal_to_null() {
    post(stubr.uri()).body(json!({"maybe": "some"})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/obj.json")]
async fn should_map_req_body_equal_to_obj() {
    post(stubr.uri())
        .body(json!({"user": {"name": "jdoe"}}))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/obj.json")]
async fn should_fail_when_req_body_key_not_equal_to_obj() {
    post(stubr.uri())
        .body(json!({"notUser": {"name": "jdoe"}}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"user": {"NotName": "jdoe"}}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/obj.json")]
async fn should_fail_when_req_body_value_not_equal_to_obj() {
    post(stubr.uri())
        .body(json!({"user": {"name": "alice"}}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/array.json")]
async fn should_map_req_body_equal_to_array() {
    post(stubr.uri())
        .body(json!({"names": ["alice", "bob"]}))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/array.json")]
async fn should_fail_when_req_body_key_not_equal_to_array() {
    post(stubr.uri())
        .body(json!({"notNames": ["alice", "bob"]}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/array.json")]
async fn should_fail_when_req_body_value_not_equal_to_array() {
    post(stubr.uri())
        .body(json!({"names": ["a", "bob"]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"names": ["alice", "b"]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"names": ["alice"]}))
        .await
        .expect_status_not_found();
    post(stubr.uri())
        .body(json!({"names": ["bob"]}))
        .await
        .expect_status_not_found();
    post(stubr.uri()).body(json!({"names": []})).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/binary.json")]
async fn should_match_req_body_equal_to_base64() {
    post(stubr.uri()).body(vec![1, 2, 3]).await.expect_status_ok();
    post(stubr.uri()).body(vec![3, 2, 1]).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/root-array.json")]
async fn should_map_req_body_equal_to_json_root_array() {
    post(stubr.uri()).body(json!(["alice", "bob"])).await.expect_status_ok();
}
