use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("resp/template/query/simple.json")]
async fn should_template_request_query_parameters() {
    get(stubr.path_queries("/api/path", ("one", "1"), ("two", "2"))).await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/query/none.json")]
async fn should_not_template_request_query_parameters_when_missing() {
    get(stubr.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("::")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/query/multi.json")]
async fn should_template_request_multi_query_parameters() {
    get(stubr.path_queries("/api/path", ("age", "1"), ("age", "2"))).await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}