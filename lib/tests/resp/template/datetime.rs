use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use asserhttp::*;
use chrono::{prelude::*, Duration, DurationRound};
use chrono_tz::Tz;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/datetime/now.json")]
async fn should_template_now_formatted_as_rfc_3339() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text_matches("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:]+Z$")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/now.json")]
async fn should_template_now_as_close_to_current_utc_current() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body| is_close_to(body, Duration::days(1), |it| it))
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/custom-format.json")]
async fn should_template_now_with_custom_format() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text_matches("^[0-9]{4}/[0-9]{2}/[0-9]{2}$")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/offset.json")]
async fn should_template_now_with_offset() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body| is_close_to(body, Duration::days(1), |resp| resp - Duration::days(3)))
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/negative-offset.json")]
async fn should_template_now_with_negative_offset() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body| is_close_to(body, Duration::days(1), |resp| resp + Duration::days(3)))
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/fmt-epoch.json")]
async fn should_template_now_with_epoch_format() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body: String| {
            let current_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let returned = u128::from_str(body.as_str()).unwrap();
            assert!(returned <= current_epoch);
            assert!(current_epoch - 1000 <= returned)
        })
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/fmt-unix.json")]
async fn should_template_now_with_unix_format() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body: String| {
            let current_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let returned = u64::from_str(body.as_str()).unwrap();
            assert!(returned <= current_epoch);
            assert!(current_epoch - 1 <= returned)
        })
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/datetime/timezone.json")]
async fn should_template_now_with_custom_timezone() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_body_text(|body| {
            is_close_to(body, Duration::hours(1), |resp| {
                let rome: Tz = "Europe/Rome".parse().unwrap();
                let naive_now: NaiveDateTime = Utc::now().naive_utc();
                let rome_offset = rome.offset_from_utc_datetime(&naive_now).fix().local_minus_utc();
                resp - Duration::seconds(rome_offset.into())
            })
        })
        .expect_content_type_text();
}

fn is_close_to(from: String, rounding: Duration, alter: fn(DateTime<Utc>) -> DateTime<Utc>) {
    let parsed = DateTime::<FixedOffset>::parse_from_rfc3339(from.as_str()).unwrap();
    let received: DateTime<Utc> = DateTime::from_utc(parsed.naive_utc(), Utc).duration_round(rounding).unwrap();
    let received = alter(received);
    let approx_now = Utc::now().duration_round(rounding).unwrap();
    assert_eq!(approx_now, received)
}
