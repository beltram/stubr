use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/status/default.json")]
async fn status_should_default_to_200() {
    get(stubr.uri()).await.expect_status(200);
}

#[async_std::test]
#[stubr::mock("resp/status/200.json")]
async fn should_map_response_status_200() {
    get(stubr.uri()).await.expect_status(200);
}

#[async_std::test]
#[stubr::mock("resp/status/400.json")]
async fn should_map_response_status_400() {
    get(stubr.uri()).await.expect_status(400);
}

#[async_std::test]
#[stubr::mock("resp/status/500.json")]
async fn should_map_response_status_500() {
    get(stubr.uri()).await.expect_status(500);
}