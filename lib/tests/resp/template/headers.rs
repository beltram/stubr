use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_header_parameters() {
    let srv = given("resp/template/headers/simple");
    get(&srv.uri()).header("a", "1").await
        .expect_status_ok()
        .expect_body_text_eq("1")
        .expect_content_type_text();
    get(&srv.uri()).header("a", "abcd").await
        .expect_status_ok()
        .expect_body_text_eq("abcd")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_not_template_request_header_parameters_when_missing() {
    let srv = given("resp/template/headers/none");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_request_multi_header_parameters() {
    let srv = given("resp/template/headers/multi");
    get(&srv.uri())
        .header("a", "1, 2")
        .await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}