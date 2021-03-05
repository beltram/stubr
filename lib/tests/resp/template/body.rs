use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_text_body() {
    let srv = given("resp/template/body/text");
    post(&srv.url()).body("Lorem ipsum").await.unwrap()
        .assert_ok()
        .assert_body_text("Lorem ipsum")
        .assert_content_type_text();
    post(&srv.url()).body("Ipsum lorem").await.unwrap()
        .assert_ok()
        .assert_body_text("Ipsum lorem")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_request_text_body_when_missing() {
    let srv = given("resp/template/body/text");
    post(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text("")
        .assert_content_type_text();
}