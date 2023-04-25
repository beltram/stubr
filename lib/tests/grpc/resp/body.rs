use crate::grpc::*;
use asserhttp::grpc::*;

pub mod simple {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/scalar.json")]
    async fn scalar_response_body() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_scalar(req)
            .await
            .expect_status_ok()
            .expect_body(Scalar {
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
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/obj.json")]
    async fn obj_response_body() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_obj(req)
            .await
            .expect_status_ok()
            .expect_body(Parent {
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
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/array.json")]
    async fn array_response_body() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_array(req)
            .await
            .expect_status_ok()
            .expect_body(Array {
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
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/optional-full.json")]
    async fn optional_response_body() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_optional(req)
            .await
            .expect_status_ok()
            .expect_body(Optional {
                opt: Some("opt".to_string()),
            });
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/optional-partial.json")]
    async fn optional_absent_response_body() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_optional(req)
            .await
            .expect_status_ok()
            .expect_body(Optional { opt: None });
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/multi-file.json")]
    async fn response_body_with_message_from_other_proto_files() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_other(req)
            .await
            .expect_status_ok()
            .expect_body(OtherParent {
                child: Some(Other {
                    name: "imported".to_string(),
                }),
            });
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/naming.json")]
    async fn response_body_should_respect_proto_naming() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .resp_naming(req)
            .await
            .expect_status_ok()
            .expect_body(Naming {
                lowercase: "lowercase".to_string(),
                uppercase: "UPPERCASE".to_string(),
                snake_case: "snake_case".to_string(),
                camel_case: "camelCase".to_string(),
            });
    }
}

pub mod template {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/template-body.json")]
    async fn should_template_request_body() {
        // should return request body by templating the 'name'
        let req = tonic::Request::new(Template { name: "alice".to_string() });
        stubr
            .connect()
            .await
            .resp_template(req)
            .await
            .expect_status_ok()
            .expect_body(Template { name: "alice".to_string() });
        // verify the response is not hardcoded
        let req = tonic::Request::new(Template { name: "bob".to_string() });
        stubr
            .connect()
            .await
            .resp_template(req)
            .await
            .expect_status_ok()
            .expect_body(Template { name: "bob".to_string() });
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/template-method.json")]
    async fn should_template_request_method() {
        // should return request method
        let req = tonic::Request::new(Template { name: "alice".to_string() });
        stubr
            .connect()
            .await
            .resp_template(req)
            .await
            .expect_status_ok()
            .expect_body(Template {
                name: "respTemplate".to_string(),
            });
    }

    #[tokio::test]
    #[stubr::mock("grpc/resp/body/template-service.json")]
    async fn should_template_request_service() {
        // should return request method
        let req = tonic::Request::new(Template { name: "alice".to_string() });
        stubr
            .connect()
            .await
            .resp_template(req)
            .await
            .expect_status_ok()
            .expect_body(Template { name: "Grpc".to_string() });
    }
}
