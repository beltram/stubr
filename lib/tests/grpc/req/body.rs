use crate::grpc::*;
use asserhttp::grpc::*;

pub mod equal_to_json {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-scalar.json")]
    async fn should_match_scalar() {
        let req = tonic::Request::new(Scalar {
            double: f64::MAX,
            float: f32::MAX,
            int32: i32::MAX,
            int64: i64::MAX,
            uint32: u32::MAX,
            uint64: u64::MAX,
            sint32: i32::MIN,
            sint64: i64::MIN,
            fixed32: u32::MAX,
            fixed64: u64::MAX,
            sfixed32: i32::MIN,
            sfixed64: i64::MIN,
            bool: true,
            string: "string".to_string(),
            bytes: b"bytes".to_vec(),
        });
        stubr
            .connect()
            .await
            .req_scalar(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-scalar.json")]
    async fn should_fail_when_scalar_mismatch() {
        let req = tonic::Request::new(Scalar {
            double: f64::MAX - 1.0,
            float: f32::MAX - 1.0,
            int32: i32::MAX - 1,
            int64: i64::MAX - 1,
            uint32: u32::MAX - 1,
            uint64: u64::MAX - 1,
            sint32: i32::MIN + 1,
            sint64: i64::MIN + 1,
            fixed32: u32::MAX - 1,
            fixed64: u64::MAX - 1,
            sfixed32: i32::MIN + 1,
            sfixed64: i64::MIN + 1,
            bool: false,
            string: "!string".to_string(),
            bytes: b"!bytes".to_vec(),
        });
        stubr.connect().await.req_scalar(req).await.expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/naming.json")]
    async fn should_match_respecting_key_names() {
        let req = tonic::Request::new(Naming {
            lowercase: "lowercase".to_string(),
            uppercase: "UPPERCASE".to_string(),
            snake_case: "snake_case".to_string(),
            camel_case: "camelCase".to_string(),
        });
        stubr
            .connect()
            .await
            .req_naming(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-optional-full.json")]
    async fn should_match_optional_when_required() {
        // matches when present
        let req = tonic::Request::new(Optional {
            opt: Some("opt".to_string()),
        });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // fails when present with incorrect value
        let req = tonic::Request::new(Optional {
            opt: Some("not-opt".to_string()),
        });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_error(Code::NotFound);

        // fails when absent
        let req = tonic::Request::new(Optional { opt: None });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-optional-partial.json")]
    async fn should_match_optional_when_absent() {
        // fails when present
        let req = tonic::Request::new(Optional {
            opt: Some("opt".to_string()),
        });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_error(Code::NotFound);

        // fails when present with incorrect value
        let req = tonic::Request::new(Optional {
            opt: Some("not-opt".to_string()),
        });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_error(Code::NotFound);

        // matches when absent
        let req = tonic::Request::new(Optional { opt: None });
        stubr
            .connect()
            .await
            .req_optional(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-array.json")]
    async fn should_match_array() {
        let req = tonic::Request::new(Array {
            double_array: vec![f64::MAX],
            float_array: vec![f32::MAX],
            int32_array: vec![i32::MAX],
            int64_array: vec![i64::MAX],
            uint32_array: vec![u32::MAX],
            uint64_array: vec![u64::MAX],
            sint32_array: vec![i32::MIN],
            sint64_array: vec![i64::MIN],
            fixed32_array: vec![u32::MAX],
            fixed64_array: vec![u64::MAX],
            sfixed32_array: vec![i32::MIN],
            sfixed64_array: vec![i64::MIN],
            bool_array: vec![true],
            string_array: vec!["string".to_string()],
            bytes_array: vec![b"bytes".to_vec()],
        });
        stubr
            .connect()
            .await
            .req_array(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-obj.json")]
    async fn should_match_obj() {
        let req = tonic::Request::new(Parent {
            child: Some(Child {
                name: "required-child".to_string(),
            }),
            maybe_child: Some(Child {
                name: "optional-child".to_string(),
            }),
            children: vec![
                Child {
                    name: "child-1".to_string(),
                },
                Child {
                    name: "child-2".to_string(),
                },
            ],
            gender: Gender::Female as i32,
        });
        stubr
            .connect()
            .await
            .req_obj(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }
}

#[tokio::test]
#[stubr::mock("grpc/req/body/multi-file.json")]
async fn should_import_message_from_other_proto_files() {
    let req = tonic::Request::new(OtherParent {
        child: Some(Other {
            name: "imported".to_string(),
        }),
    });
    stubr
        .connect()
        .await
        .req_other(req)
        .await
        .expect_status_ok()
        .expect_body(Empty::default());
}

pub mod equal_to_json_relaxed {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-relaxed.json")]
    async fn should_ignore_array_order() {
        // ordered
        let req = tonic::Request::new(Relaxed {
            names: vec!["alice".to_string(), "bob".to_string()],
            nickname: None,
        });
        stubr
            .connect()
            .await
            .req_relaxed(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // unordered
        let req = tonic::Request::new(Relaxed {
            names: vec!["bob".to_string(), "alice".to_string()],
            nickname: None,
        });
        stubr
            .connect()
            .await
            .req_relaxed(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // cannot have extra element
        let req = tonic::Request::new(Relaxed {
            names: vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()],
            nickname: None,
        });
        stubr.connect().await.req_relaxed(req).await.expect_status_error(Code::NotFound);

        // cannot be missing an element
        let req = tonic::Request::new(Relaxed {
            names: vec!["alice".to_string()],
            nickname: None,
        });
        stubr.connect().await.req_relaxed(req).await.expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/eq-relaxed.json")]
    async fn should_allow_extra_field() {
        // missing as in the stub => ok
        let req = tonic::Request::new(Relaxed {
            names: vec!["alice".to_string(), "bob".to_string()],
            nickname: None,
        });
        stubr
            .connect()
            .await
            .req_relaxed(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // present while missing in stub => also ok
        let req = tonic::Request::new(Relaxed {
            names: vec!["alice".to_string(), "bob".to_string()],
            nickname: Some("crypto".to_string()),
        });
        stubr
            .connect()
            .await
            .req_relaxed(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }
}

pub mod json_path {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/body/json-path.json")]
    async fn should_match_json_path() {
        let req = tonic::Request::new(JsonPath {
            name: Some("alice".to_string()),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/json-path.json")]
    async fn should_fail_when_json_path_absent() {
        let req = tonic::Request::new(JsonPath {
            name: None,
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/json-path-snake-case.json")]
    async fn should_match_json_path_in_snake_case() {
        // matches `"matchesJsonPath": "$.composed_name"`
        let req = tonic::Request::new(JsonPath {
            composed_name: Some("alice doe".to_string()),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // at least verify that it fails when absent
        let req = tonic::Request::new(JsonPath {
            composed_name: None,
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/json-path-contains.json")]
    async fn should_match_json_path_contains() {
        // should match `"matchesJsonPath": "$.name", "contains": "a"`
        let req = tonic::Request::new(JsonPath {
            name: Some("alice".to_string()),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // should fail
        let req = tonic::Request::new(JsonPath {
            name: Some("bob".to_string()),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/json-path-eq.json")]
    async fn should_match_json_path_equal_to_json() {
        // should match `"matchesJsonPath": "$.child", "equalToJson": {"name": "bob"}`
        let req = tonic::Request::new(JsonPath {
            child: Some(Child { name: "bob".to_string() }),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // should fail
        let req = tonic::Request::new(JsonPath {
            child: Some(Child { name: "alice".to_string() }),
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_error(Code::NotFound);

        // should fail when absent
        let req = tonic::Request::new(JsonPath {
            child: None,
            ..Default::default()
        });
        stubr
            .connect()
            .await
            .req_json_path(req)
            .await
            .expect_status_error(Code::NotFound);
    }
}

pub mod binary {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/body/binary.json")]
    async fn should_match_binary() {
        let body = Child { name: "joe".to_string() };
        let req = tonic::Request::new(body);
        stubr
            .connect()
            .await
            .req_binary(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/body/binary.json")]
    async fn should_fail_when_mismatch() {
        let body = Child { name: "jim".to_string() };
        let req = tonic::Request::new(body);
        stubr.connect().await.req_binary(req).await.expect_status_error(Code::NotFound);
    }
}
