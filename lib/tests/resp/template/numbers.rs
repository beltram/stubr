use asserhttp::*;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/numbers/odd.json")]
async fn should_template_is_odd() {
    post(stubr.uri())
        .body("3")
        .await
        .expect_status_ok()
        .expect_body_text_eq("true")
        .expect_content_type_text();
    post(stubr.uri())
        .body("4")
        .await
        .expect_status_ok()
        .expect_body_text_eq("false")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/numbers/even.json")]
async fn should_template_is_even() {
    post(stubr.uri())
        .body("3")
        .await
        .expect_status_ok()
        .expect_body_text_eq("false")
        .expect_content_type_text();
    post(stubr.uri())
        .body("4")
        .await
        .expect_status_ok()
        .expect_body_text_eq("true")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/numbers/stripes.json")]
async fn should_template_stripes() {
    post(stubr.uri())
        .body("4")
        .await
        .expect_status_ok()
        .expect_body_text_eq("if-even")
        .expect_content_type_text();
    post(stubr.uri())
        .body("5")
        .await
        .expect_status_ok()
        .expect_body_text_eq("if-odd")
        .expect_content_type_text();
}
