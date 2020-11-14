use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_request_method_get() {
    let srv = given("req/method/get");
    surf::get(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_post() {
    let srv = given("req/method/post");
    surf::post(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_put() {
    let srv = given("req/method/put");
    surf::put(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_delete() {
    let srv = given("req/method/delete");
    surf::delete(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_patch() {
    let srv = given("req/method/patch");
    surf::patch(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_options() {
    let srv = given("req/method/options");
    surf::options(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_connect() {
    let srv = given("req/method/connect");
    surf::connect(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_map_request_method_trace() {
    let srv = given("req/method/trace");
    surf::trace(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_invalid_method() {
    let srv = given("req/method/get");
    surf::post(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_any_request_method() {
    let srv = given("req/method/any");
    surf::get(&srv.uri()).await.unwrap().assert_ok();
    surf::post(&srv.uri()).await.unwrap().assert_ok();
    surf::put(&srv.uri()).await.unwrap().assert_ok();
    surf::delete(&srv.uri()).await.unwrap().assert_ok();
    surf::patch(&srv.uri()).await.unwrap().assert_ok();
    surf::options(&srv.uri()).await.unwrap().assert_ok();
    surf::connect(&srv.uri()).await.unwrap().assert_ok();
    surf::trace(&srv.uri()).await.unwrap().assert_ok();
}
