use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn status_should_default_to_200() {
    let srv = given("resp/status/default");
    get(&srv.uri()).await.expect_status_eq(200);
}

#[async_std::test]
async fn should_map_response_status_200() {
    let srv = given("resp/status/200");
    get(&srv.uri()).await.expect_status_eq(200);
}

#[async_std::test]
async fn should_map_response_status_400() {
    let srv = given("resp/status/400");
    get(&srv.uri()).await.expect_status_eq(400);
}

#[async_std::test]
async fn should_map_response_status_500() {
    let srv = given("resp/status/500");
    get(&srv.uri()).await.expect_status_eq(500);
}