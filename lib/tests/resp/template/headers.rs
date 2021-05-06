use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_header_parameters() {
    let srv = given("resp/template/headers/simple");
    get(&srv.uri()).header("a", "1").await.unwrap()
        .assert_ok()
        .assert_body_text("1")
        .assert_content_type_text();
    get(&srv.uri()).header("a", "abcd").await.unwrap()
        .assert_ok()
        .assert_body_text("abcd")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_request_header_parameters_when_missing() {
    let srv = given("resp/template/headers/none");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text("")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_request_multi_header_parameters() {
    let srv = given("resp/template/headers/multi");
    get(&srv.uri())
        .header("a", "1, 2")
        .await.unwrap()
        .assert_ok()
        .assert_body_text("1::2")
        .assert_content_type_text();
}