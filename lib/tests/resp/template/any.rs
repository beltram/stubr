use asserhttp::*;
use regex::Regex;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/any/regex.json")]
async fn should_template_any_regex() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| {
            assert!(Regex::new("[0-9]{5}[a-z]{5}").unwrap().is_match(&b));
            assert!(!b.starts_with('\''));
            assert!(!b.ends_with('\''));
        });
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

#[async_std::test]
#[stubr::mock("resp/template/any/alpha-numeric.json")]
async fn should_template_any_alpha_numeric() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.chars().all(|c| c.is_alphanumeric())));
}

#[async_std::test]
#[stubr::mock("resp/template/any/number.json")]
async fn should_template_any_number() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.parse::<i64>().is_ok() || b.parse::<f64>().is_ok()));
}

#[async_std::test]
#[stubr::mock("resp/template/any/integer.json")]
async fn should_template_any_integer() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.parse::<i64>().is_ok()));
}

#[async_std::test]
#[stubr::mock("resp/template/any/float.json")]
async fn should_template_any_float() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.parse::<f64>().is_ok()));
}