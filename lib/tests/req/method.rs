use crate::utils::*;

#[async_std::test]
async fn should_map_request_method_get() {
    let srv = given("req/method/get");
    surf::get(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_post() {
    let srv = given("req/method/post");
    surf::post(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_put() {
    let srv = given("req/method/put");
    surf::put(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_delete() {
    let srv = given("req/method/delete");
    surf::delete(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_patch() {
    let srv = given("req/method/patch");
    surf::patch(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_options() {
    let srv = given("req/method/options");
    surf::options(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_connect() {
    let srv = given("req/method/connect");
    surf::connect(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_trace() {
    let srv = given("req/method/trace");
    surf::trace(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_invalid_method() {
    let srv = given("req/method/get");
    surf::post(&srv.url()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_any_request_method() {
    let srv = given("req/method/any");
    surf::get(&srv.url()).await.unwrap().assert_ok();
    surf::post(&srv.url()).await.unwrap().assert_ok();
    surf::put(&srv.url()).await.unwrap().assert_ok();
    surf::delete(&srv.url()).await.unwrap().assert_ok();
    surf::patch(&srv.url()).await.unwrap().assert_ok();
    surf::options(&srv.url()).await.unwrap().assert_ok();
    surf::connect(&srv.url()).await.unwrap().assert_ok();
    surf::trace(&srv.url()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_default_to_any() {
    let srv = given("req/method/missing");
    surf::get(&srv.url()).await.unwrap().assert_ok();
    surf::post(&srv.url()).await.unwrap().assert_ok();
    surf::put(&srv.url()).await.unwrap().assert_ok();
    surf::delete(&srv.url()).await.unwrap().assert_ok();
    surf::patch(&srv.url()).await.unwrap().assert_ok();
    surf::options(&srv.url()).await.unwrap().assert_ok();
    surf::connect(&srv.url()).await.unwrap().assert_ok();
    surf::trace(&srv.url()).await.unwrap().assert_ok();
}
