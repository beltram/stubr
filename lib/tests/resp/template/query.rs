use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_query_parameters() {
    let srv = given("resp/template/query/simple");
    get(&srv.path_queries("/api/path", ("one", "1"), ("two", "2"))).await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_not_template_request_query_parameters_when_missing() {
    let srv = given("resp/template/query/none");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("::")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_request_multi_query_parameters() {
    let srv = given("resp/template/query/multi");
    get(&srv.path_queries("/api/path", ("age", "1"), ("age", "2"))).await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}