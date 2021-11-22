use asserhttp::*;
use serde_json::json;
use surf::post;

mod simple {
    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_text() {
        let body = json!({"any": "text"});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_bool() {
        let body = json!({"any": true});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_int() {
        let body = json!({"any": 42});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_float() {
        let body = json!({"any": 1.2});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_null() {
        let body = json!({"any": null});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }
}

mod object {
    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_object() {
        let body = json!({"any": {"name": "jdoe"}});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/obj-nested.json")]
    async fn should_template_nested_object() {
        post(stubr.uri()).body(json!({"any": "jdoe"})).await
            .expect_status_ok()
            .expect_body_json_eq(json!({"any": {"nested": "jdoe"}}))
            .expect_content_type_json();
    }
}

mod array {
    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_array() {
        let body = json!({"any": ["a", "b"]});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/simple.json")]
    async fn should_template_complex_array() {
        let body = json!({"any": [{"name": "alice"}, {"name": "bob"}]});
        post(stubr.uri()).body(body.clone()).await
            .expect_status_ok()
            .expect_body_json_eq(body)
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/array-item.json")]
    async fn should_template_within_array_item() {
        post(stubr.uri()).body(json!({"a": "jdoe"})).await
            .expect_status_ok()
            .expect_body_json_eq(json!({"array": ["jdoe"]}))
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/array-item.json")]
    async fn should_template_array_within_array_item() {
        post(stubr.uri()).body(json!({"a": ["alice", "bob"]})).await
            .expect_status_ok()
            .expect_body_json_eq(json!({"array": [["alice", "bob"]]}))
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/array-complex-item.json")]
    async fn should_template_within_array_complex_item() {
        post(stubr.uri()).body(json!({"a": "jdoe"})).await
            .expect_status_ok()
            .expect_body_json_eq(json!({"array": [{"item": "jdoe"}]}))
            .expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("resp/template/body-type/root-array.json")]
    async fn should_template_root_json_array() {
        post(stubr.uri()).body(json!({"a": "alice", "b": "bob"})).await
            .expect_status_ok()
            .expect_body_json_eq(json!(["alice", "bob"]))
            .expect_content_type_json();
    }
}
