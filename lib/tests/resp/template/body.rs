use asserhttp::*;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_text_body() {
    let srv = given("resp/template/body/text");
    post(&srv.uri()).body("Lorem ipsum").await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum")
        .expect_content_type_text();
    post(&srv.uri()).body("Ipsum lorem").await
        .expect_status_ok()
        .expect_body_text_eq("Ipsum lorem")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_not_template_request_text_body_when_missing() {
    let srv = given("resp/template/body/text");
    post(&srv.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}