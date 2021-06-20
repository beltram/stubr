use asserhttp::*;

#[async_std::test]
#[stubr::mock("req/method/get.json")]
async fn should_map_request_method_get() {
    surf::get(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/post.json")]
async fn should_map_request_method_post() {
    surf::post(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/put.json")]
async fn should_map_request_method_put() {
    surf::put(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/delete.json")]
async fn should_map_request_method_delete() {
    surf::delete(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/patch.json")]
async fn should_map_request_method_patch() {
    surf::patch(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/options.json")]
async fn should_map_request_method_options() {
    surf::options(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/connect.json")]
async fn should_map_request_method_connect() {
    surf::connect(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/trace.json")]
async fn should_map_request_method_trace() {
    surf::trace(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/get.json")]
async fn should_fail_when_invalid_method() {
    surf::post(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/method/any.json")]
async fn should_map_any_request_method() {
    surf::get(stubr.uri()).await.expect_status_ok();
    surf::post(stubr.uri()).await.expect_status_ok();
    surf::put(stubr.uri()).await.expect_status_ok();
    surf::delete(stubr.uri()).await.expect_status_ok();
    surf::patch(stubr.uri()).await.expect_status_ok();
    surf::options(stubr.uri()).await.expect_status_ok();
    surf::connect(stubr.uri()).await.expect_status_ok();
    surf::trace(stubr.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/method/missing.json")]
async fn should_default_to_any() {
    surf::get(stubr.uri()).await.expect_status_ok();
    surf::post(stubr.uri()).await.expect_status_ok();
    surf::put(stubr.uri()).await.expect_status_ok();
    surf::delete(stubr.uri()).await.expect_status_ok();
    surf::patch(stubr.uri()).await.expect_status_ok();
    surf::options(stubr.uri()).await.expect_status_ok();
    surf::connect(stubr.uri()).await.expect_status_ok();
    surf::trace(stubr.uri()).await.expect_status_ok();
}
