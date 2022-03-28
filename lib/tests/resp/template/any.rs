use asserhttp::*;
use regex::Regex;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/any/regex.json")]
async fn should_template_any_regex() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(Regex::new("[0-9]{5}[a-z]{5}").unwrap().is_match(&b)));
}

#[async_std::test]
#[stubr::mock("resp/template/any/non-blank-string.json")]
async fn should_template_any_non_blank_string() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| {
            assert!(!b.chars().all(|c| c == ' '));
            assert!(b.len().gt(&0));
        });
}

#[async_std::test]
#[stubr::mock("resp/template/any/non-empty-string.json")]
async fn should_template_any_non_empty_string() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.len().gt(&0)));
}