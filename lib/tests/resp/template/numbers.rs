use asserhttp::*;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_template_is_odd() {
    let srv = given("resp/template/numbers/odd");
    post(&srv.uri()).body("3").await
        .expect_status_ok()
        .expect_body_text_eq("true")
        .expect_content_type_text();
    post(&srv.uri()).body("4").await
        .expect_status_ok()
        .expect_body_text_eq("false")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_is_even() {
    let srv = given("resp/template/numbers/even");
    post(&srv.uri()).body("3").await
        .expect_status_ok()
        .expect_body_text_eq("false")
        .expect_content_type_text();
    post(&srv.uri()).body("4").await
        .expect_status_ok()
        .expect_body_text_eq("true")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_stripes() {
    let srv = given("resp/template/numbers/stripes");
    post(&srv.uri()).body("4").await
        .expect_status_ok()
        .expect_body_text_eq("if-even")
        .expect_content_type_text();
    post(&srv.uri()).body("5").await
        .expect_status_ok()
        .expect_body_text_eq("if-odd")
        .expect_content_type_text();
}