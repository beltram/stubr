use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/headers/single.json")]
async fn should_return_single_response_header() {
    get(stubr.uri()).await.expect_status_ok().expect_header("X-Header-1", "1");
}

#[async_std::test]
#[stubr::mock("resp/headers/many.json")]
async fn should_return_many_response_header() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_header("X-Header-1", "1")
        .expect_header("X-Header-2", "2");
}

#[async_std::test]
#[stubr::mock("resp/headers/none.json")]
async fn should_not_return_header_when_absent() {
    get(stubr.uri()).await.expect_status_ok().expect_header_absent("x-header-1");
}

#[async_std::test]
#[stubr::mock("resp/headers/server.json")]
async fn user_defined_server_header_should_have_precedence_over_default_one() {
    get(stubr.uri()).await.expect_status_ok().expect_header("Server", "my-app");
}

#[async_std::test]
#[stubr::mock("resp/headers/cache-control.json")]
async fn should_not_supersede_cache_control() {
    get(stubr.uri())
        .header("cache-control", "a, b")
        .await
        .expect_status_ok()
        .expect_header("cache-control", "explicit");
}
