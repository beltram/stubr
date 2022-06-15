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
#[stubr::mock("resp/template/any/uuid.json")]
async fn should_template_any_uuid() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(uuid::Uuid::parse_str(&b).is_ok()));
}

#[async_std::test]
#[stubr::mock("resp/template/any/email.json")]
async fn should_template_any_email() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| {
            assert!(email_address_parser::EmailAddress::parse(&b, None).is_some())
        });
}

#[async_std::test]
#[stubr::mock("resp/template/any/host.json")]
async fn should_template_any_hostname() {
    let regex = regex::Regex::new(r"((http[s]?|ftp):/)/?([^:/\s]+)(:[0-9]{1,5})?").unwrap();
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(regex.is_match(&b)));
}

#[async_std::test]
#[stubr::mock("resp/template/any/ip.json")]
async fn should_template_any_ip_address() {
    let regex = regex::Regex::new(r"([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])").unwrap();
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(regex.is_match(&b)));
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
#[stubr::mock("resp/template/any/float.json")]
async fn should_template_any_float() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.parse::<f64>().is_ok()));
}

#[async_std::test]
#[stubr::mock("resp/template/any/bool.json")]
async fn should_template_any_boolean() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(b.parse::<bool>().is_ok()));
}

#[async_std::test]
#[stubr::mock("resp/template/any/of.json")]
async fn should_template_any_of() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text(|b: String| assert!(&b == "A" || &b == "B" || &b == "C"));
}

mod int {
    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/i64.json")]
    async fn should_template_any_i64() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<i64>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/u64.json")]
    async fn should_template_any_u64() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<u64>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/i32.json")]
    async fn should_template_any_i32() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<i32>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/u32.json")]
    async fn should_template_any_u32() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<u32>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/i16.json")]
    async fn should_template_any_i16() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<i16>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/u16.json")]
    async fn should_template_any_u16() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<u16>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/i8.json")]
    async fn should_template_any_i8() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<i8>().is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/int/u8.json")]
    async fn should_template_any_u8() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(b.parse::<u8>().is_ok()));
    }
}

mod datetime {
    use std::str::FromStr;

    use super::*;

    #[async_std::test]
    #[stubr::mock("resp/template/any/time.json")]
    async fn should_template_any_time() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(chrono::NaiveTime::from_str(&b).is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/date.json")]
    async fn should_template_any_date() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(chrono::NaiveDate::from_str(&b).is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/date-time.json")]
    async fn should_template_any_datetime() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| assert!(chrono::NaiveDateTime::from_str(&b).is_ok()));
    }

    #[async_std::test]
    #[stubr::mock("resp/template/any/iso8601.json")]
    async fn should_template_any_iso_8601_datetime() {
        get(stubr.uri()).await
            .expect_status_ok()
            .expect_content_type_text()
            .expect_body_text(|b: String| {
                let dt: Option<chrono::DateTime<chrono::Utc>> = chrono::DateTime::from_str(&b).ok();
                assert!(dt.is_some())
            });
    }
}

#[async_std::test]
#[stubr::mock("resp/template/any/header.json")]
async fn should_template_any_in_header() {
    let response = get(stubr.uri()).await.unwrap();
    let header = response.header("x-header").and_then(|h| h.get(0)).unwrap().as_str();
    assert!(Regex::new("[0-9]{5}[a-z]{5}").unwrap().is_match(header));
    assert!(!header.starts_with('\''));
    assert!(!header.ends_with('\''));
}