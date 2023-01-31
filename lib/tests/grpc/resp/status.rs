use crate::grpc::*;
use asserhttp::grpc::*;
use tonic::Request;

fn req() -> Request<Empty> {
    Request::new(Empty::default())
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/ok.json")]
async fn ok() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_ok()
        .expect_body(Empty::default());
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/cancelled.json")]
async fn cancelled() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::Cancelled);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/unknown.json")]
async fn unknown() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::Unknown);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/invalid-argument.json")]
async fn invalid_argument() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::InvalidArgument);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/deadline-exceeded.json")]
async fn deadline_exceeded() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::DeadlineExceeded);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/not-found.json")]
async fn not_found() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::NotFound);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/already-exists.json")]
async fn already_exists() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::AlreadyExists);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/permission-denied.json")]
async fn permission_denied() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::PermissionDenied);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/resource-exhausted.json")]
async fn resource_exhausted() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::ResourceExhausted);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/failed-precondition.json")]
async fn failed_precondition() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::FailedPrecondition);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/aborted.json")]
async fn aborted() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::Aborted);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/out-of-range.json")]
async fn out_of_range() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::OutOfRange);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/unimplemented.json")]
async fn unimplemented() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::Unimplemented);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/internal.json")]
async fn internal() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::Internal);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/unavailable.json")]
async fn unavailable() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::Unavailable);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/data-loss.json")]
async fn data_loss() {
    stubr.connect().await.status(req()).await.expect_status_error(Code::DataLoss);
}

#[tokio::test]
#[stubr::mock("grpc/resp/status/unauthenticated.json")]
async fn unauthenticated() {
    stubr
        .connect()
        .await
        .status(req())
        .await
        .expect_status_error(Code::Unauthenticated);
}
