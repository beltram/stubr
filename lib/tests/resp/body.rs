use asserhttp::*;
use serde_json::{Map, Value};
use serde_json::json;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/body/text.json")]
async fn should_map_text_response_body() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("Hello World !")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/body/text-blank.json")]
async fn should_map_blank_text_response_body() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq(" ")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/body/text-empty.json")]
async fn should_map_empty_text_response_body() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/body/json.json")]
async fn should_map_json_response_body() {
    let expected = json!({"name": "john", "age": 42, "candidate": true, "surnames": ["jdoe", "johnny"]});
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(expected)
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/body/json-empty.json")]
async fn should_map_empty_json_response_body() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(Value::Object(Map::default()))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/body/body-file-json.json")]
async fn from_file_should_map_from_json_file() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"name": "jdoe", "age": 4}))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/body/body-file-txt.json")]
async fn from_file_should_map_from_txt_file() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("jdoe,4")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/body/body-file-not-path.json")]
async fn from_file_should_fail_when_not_a_valid_path() {
    get(stubr.uri()).await.expect_status_internal_server_error();
}

#[async_std::test]
#[stubr::mock("resp/body/body-file-not-existing.json")]
async fn from_file_should_fail_when_file_does_not_exist() {
    get(stubr.uri()).await.expect_status_internal_server_error();
}

#[async_std::test]
#[stubr::mock("resp/body/body-file-invalid-json.json")]
async fn from_file_should_fail_when_invalid_json_in_file() {
    get(stubr.uri()).await.expect_status_internal_server_error();
}
