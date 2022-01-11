use asserhttp::*;
use serde_json::json;
use surf::get;

mod text {
    use super::*;

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
}

mod binary {
    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/body/binary.json")]
    async fn should_map_binary_response_body() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_header("content-type", "application/octet-stream")
            .expect_body_text_eq("beltram");
    }

    #[async_std::test]
    #[stubr::mock("resp/body/binary-empty.json")]
    async fn should_map_empty_binary_response_body() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_header("content-type", "application/octet-stream")
            .expect_body_absent();
    }


    #[async_std::test]
    #[stubr::mock("resp/body/binary-template.json")]
    async fn should_map_binary_response_body_even_when_response_template_requested() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_header("content-type", "application/octet-stream")
            .expect_body_text_eq("beltram");
    }

    #[async_std::test]
    #[stubr::mock("resp/body/binary-json.json")]
    async fn should_preserve_explicit_content_type() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_json()
            .expect_body_json_eq(json!({"name":"john"}));
    }
}

mod json {
    use super::*;

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
            .expect_body_json_eq(json!({}))
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/body/json-array.json")]
    async fn should_map_json_array_response_body() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_body_json_eq(json!(["alice", "bob"]))
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/body/empty-json-array.json")]
    async fn should_map_empty_json_array_response_body() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_body_json_eq(json!([]))
            .expect_content_type_json();
    }
}

mod file {
    use super::*;

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
}