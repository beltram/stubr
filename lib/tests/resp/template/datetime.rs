use std::{str::FromStr, time::{SystemTime, UNIX_EPOCH}};

use chrono::{Duration, DurationRound, prelude::*};
use chrono_tz::Tz;
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
        .assert_body_text_satisfies(|body| is_close_to(body, Duration::days(1), |it| it))
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

#[async_std::test]
async fn should_template_now_with_offset() {
    let srv = given("resp/template/datetime/offset");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| is_close_to(body, Duration::days(1), |resp| resp - Duration::days(3)))
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_with_negative_offset() {
    let srv = given("resp/template/datetime/negative-offset");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| is_close_to(body, Duration::days(1), |resp| resp + Duration::days(3)))
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_with_epoch_format() {
    let srv = given("resp/template/datetime/fmt-epoch");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| {
            let current_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let returned = u128::from_str(body).unwrap();
            assert!(returned <= current_epoch);
            assert!(current_epoch - 1000 <= returned)
        })
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_with_unix_format() {
    let srv = given("resp/template/datetime/fmt-unix");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| {
            let current_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let returned = u64::from_str(body).unwrap();
            assert!(returned <= current_epoch);
            assert!(current_epoch - 1 <= returned)
        })
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_now_with_custom_timezone() {
    let srv = given("resp/template/datetime/timezone");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text_satisfies(|body| is_close_to(body, Duration::hours(1), |resp| {
            let rome: Tz = "Europe/Rome".parse().unwrap();
            let naive_now: NaiveDateTime = Utc::now().naive_utc();
            let rome_offset = rome.offset_from_utc_datetime(&naive_now).fix().local_minus_utc();
            resp - Duration::seconds(rome_offset.into())
        }))
        .assert_content_type_text();
}

fn is_close_to(from: &str, rounding: Duration, alter: fn(DateTime<Utc>) -> DateTime<Utc>) {
    let parsed = DateTime::<FixedOffset>::parse_from_rfc3339(from).unwrap();
    let received: DateTime<Utc> = DateTime::from_utc(parsed.naive_utc(), Utc)
        .duration_round(rounding).unwrap();
    let received = alter(received);
    let approx_now = Utc::now().duration_round(rounding).unwrap();
    assert_eq!(approx_now, received)
}