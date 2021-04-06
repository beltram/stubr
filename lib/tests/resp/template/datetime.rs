use chrono::{Duration, DurationRound, prelude::*};
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_now_formatted_as_rfc_3339() {
    let srv = given("resp/template/datetime/now");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_matches("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:]+Z$")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_as_close_to_current_utc_current() {
    let srv = given("resp/template/datetime/now");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| is_close_to(body, Duration::days(1)))
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_with_custom_format() {
    let srv = given("resp/template/datetime/custom-format");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_matches("^[0-9]{4}/[0-9]{2}/[0-9]{2}$")
        .assert_content_type_text();
}

fn is_close_to(from: &str, rounding: Duration) {
    let approx_now = Utc::now().duration_round(rounding).unwrap();
    let parsed = DateTime::<FixedOffset>::parse_from_rfc3339(from).unwrap();
    let received: DateTime<Utc> = DateTime::from_utc(parsed.naive_utc(), Utc)
        .duration_round(rounding).unwrap();
    assert_eq!(approx_now, received)
}