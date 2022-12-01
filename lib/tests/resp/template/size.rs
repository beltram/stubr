use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/size/single.json")]
async fn should_return_size_of_string() {
    post(stubr.uri())
        .body("abcd")
        .await
        .expect_status_ok()
        .expect_body_text_eq("4")
        .expect_content_type_text();
    post(stubr.uri())
        .body("abcdefgh")
        .await
        .expect_status_ok()
        .expect_body_text_eq("8")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/size/single.json")]
async fn should_return_size_of_json_body_keys() {
    post(stubr.uri())
        .body(json!({"a": "b"}))
        .await
        .expect_status_ok()
        .expect_body_text_eq("1")
        .expect_content_type_text();
    post(stubr.uri())
        .body(json!({"a": "b", "c": "d"}))
        .await
        .expect_status_ok()
        .expect_body_text_eq("2")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/size/single.json")]
async fn should_return_size_of_json_array_field() {
    post(stubr.uri())
        .body(json!(["alice", "bob"]))
        .await
        .expect_status_ok()
        .expect_body_text_eq("2")
        .expect_content_type_text();
    post(stubr.uri())
        .body(json!(["alice", "bob", "john"]))
        .await
        .expect_status_ok()
        .expect_body_text_eq("3")
        .expect_content_type_text();
}
