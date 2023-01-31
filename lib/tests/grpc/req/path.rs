use crate::grpc::*;
use asserhttp::grpc::*;

#[tokio::test]
#[stubr::mock("grpc/req/path/eq.json")]
async fn should_match_path() {
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
#[stubr::mock("grpc/req/path/eq.json")]
async fn should_fail_when_path_mismatch() {
    let req = tonic::Request::new(Empty::default());
    stubr
        .connect()
        .await
        .req_path_not_eq(req)
        .await
        .expect_status_error(Code::NotFound);
}

#[tokio::test]
#[stubr::mock("grpc/req/path/no-path.json")]
async fn should_match_when_no_path_defined() {
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
#[stubr::mock("grpc/req/path/regex.json")]
async fn should_match_regex() {
    // both paths match `reqPathEq(.*)`
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
