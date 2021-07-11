use asserhttp::*;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/string/capitalize.json")]
async fn should_template_capitalize() {
    post(stubr.uri()).body("john").await
        .expect_status_ok()
        .expect_body_text_eq("John")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/string/decapitalize.json")]
async fn should_template_decapitalize() {
    post(stubr.uri()).body("JOHN").await
        .expect_status_ok()
        .expect_body_text_eq("jOHN")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/string/upper.json")]
async fn should_template_uppercase() {
    post(stubr.uri()).body("john").await
        .expect_status_ok()
        .expect_body_text_eq("JOHN")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/string/lower.json")]
async fn should_template_lowercase() {
    post(stubr.uri()).body("JOHN").await
        .expect_status_ok()
        .expect_body_text_eq("john")
        .expect_content_type_text();
}