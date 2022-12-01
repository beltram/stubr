use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/extra/on.json")]
async fn should_match_req_body_equal_to_json_ignoring_extra_elements() {
    post(stubr.uri())
        .body(json!({"name": "juste", "age": 42}))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/extra/on.json")]
async fn should_not_match_req_body_equal_to_json_ignoring_extra_elements_when_key_mismatches() {
    post(stubr.uri())
        .body(json!({"not-name": "juste", "age": 42}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/extra/on.json")]
async fn should_not_match_req_body_equal_to_json_ignoring_extra_elements_when_value_mismatches() {
    post(stubr.uri())
        .body(json!({"name": "not-juste", "age": 42}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/extra/off.json")]
async fn should_match_req_body_equal_to_json_not_ignoring_extra_elements() {
    post(stubr.uri())
        .body(json!({"name": "juste", "age": 42}))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/body/eq/ignore/extra/default.json")]
async fn by_default_should_match_req_body_equal_to_json_not_ignoring_extra_elements() {
    post(stubr.uri())
        .body(json!({"name": "juste", "age": 42}))
        .await
        .expect_status_not_found();
}
