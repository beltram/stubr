use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/headers/simple.json")]
async fn should_template_request_header_parameters() {
    get(stubr.uri())
        .header("a", "1")
        .await
        .expect_status_ok()
        .expect_body_text_eq("1")
        .expect_content_type_text();
    get(stubr.uri())
        .header("a", "abcd")
        .await
        .expect_status_ok()
        .expect_body_text_eq("abcd")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/headers/none.json")]
async fn should_not_template_request_header_parameters_when_missing() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/headers/multi.json")]
async fn should_template_request_multi_header_parameters() {
    get(stubr.uri())
        .header("a", "1, 2")
        .await
        .expect_status_ok()
        .expect_body_text_eq("1::2")
        .expect_content_type_text();
}
