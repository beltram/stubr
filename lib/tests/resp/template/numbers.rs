use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_template_is_odd() {
    let srv = given("resp/template/numbers/odd");
    post(&srv.uri()).body("3").await.unwrap()
        .assert_ok()
        .assert_body_text("true")
        .assert_content_type_text();
    post(&srv.uri()).body("4").await.unwrap()
        .assert_ok()
        .assert_body_text("false")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_is_even() {
    let srv = given("resp/template/numbers/even");
    post(&srv.uri()).body("3").await.unwrap()
        .assert_ok()
        .assert_body_text("false")
        .assert_content_type_text();
    post(&srv.uri()).body("4").await.unwrap()
        .assert_ok()
        .assert_body_text("true")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_stripes() {
    let srv = given("resp/template/numbers/stripes");
    post(&srv.uri()).body("4").await.unwrap()
        .assert_ok()
        .assert_body_text("if-even")
        .assert_content_type_text();
    post(&srv.uri()).body("5").await.unwrap()
        .assert_ok()
        .assert_body_text("if-odd")
        .assert_content_type_text();
}