use asserhttp::*;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/body/text.json")]
async fn should_template_request_text_body() {
    post(stubr.uri()).body("Lorem ipsum").await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum")
        .expect_content_type_text();
    post(stubr.uri()).body("Ipsum lorem").await
        .expect_status_ok()
        .expect_body_text_eq("Ipsum lorem")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/body/text.json")]
async fn should_not_template_request_text_body_when_missing() {
    post(stubr.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}