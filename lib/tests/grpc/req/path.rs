use crate::grpc::*;
use asserhttp::grpc::*;

mod endpoint {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/method/eq.json")]
    async fn should_match_method() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/method/eq.json")]
    async fn should_fail_when_method_mismatch() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_not_eq(req)
            .await
            .expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/method/absent.json")]
    async fn should_match_when_no_method_defined() {
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_not_eq(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/method/regex.json")]
    async fn should_match_regex() {
        // both methods match `reqPathEq(.*)`
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_eq_regex(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());

        // `reqPathNotEq` does not match `reqPathEq(.*)`
        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_not_eq(req)
            .await
            .expect_status_error(Code::NotFound);
    }
}

mod svc {
    use super::*;

    #[tokio::test]
    #[stubr::mock("grpc/req/svc/eq.json")]
    async fn should_match_svc_name() {
        let req = tonic::Request::new(EmptyOther::default());
        stubr
            .connect_other()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(EmptyOther::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/svc/eq.json")]
    async fn should_fail_when_svc_mismatch() {
        let req = tonic::Request::new(Empty::default());
        stubr.connect().await.req_path_eq(req).await.expect_status_error(Code::NotFound);
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/svc/absent.json")]
    async fn should_match_when_no_svc_defined() {
        let req = tonic::Request::new(EmptyOther::default());
        stubr
            .connect_other()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(EmptyOther::default());

        let req = tonic::Request::new(Empty::default());
        stubr
            .connect()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(Empty::default());
    }

    #[tokio::test]
    #[stubr::mock("grpc/req/svc/regex.json")]
    async fn should_match_regex() {
        let req = tonic::Request::new(EmptyOther::default());
        stubr
            .connect_other()
            .await
            .req_path_eq(req)
            .await
            .expect_status_ok()
            .expect_body(EmptyOther::default());
        let req = tonic::Request::new(EmptyOther::default());
        stubr
            .connect_other()
            .await
            .req_path_eq_regex(req)
            .await
            .expect_status_ok()
            .expect_body(EmptyOther::default());
    }
}
