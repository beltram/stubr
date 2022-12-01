use asserhttp::*;
use isahc::Request;
use serde_json::json;

use stubr::*;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/get.json")]
async fn proxy_should_forward_get_method() {
    isahc::get(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .get(stubr.uri())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "9143887468928371923",
        json!({
            "request": {"method": "GET"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/post.json")]
async fn proxy_should_forward_post_method() {
    isahc::post(stubr.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .post(stubr.uri(), ())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "11472728421990930279",
        json!({
            "request": {"method": "POST"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/put.json")]
async fn proxy_should_forward_put_method() {
    isahc::put(stubr.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .put(stubr.uri(), ())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "1225390682374767577",
        json!({
            "request": {"method": "PUT"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/patch.json")]
async fn proxy_should_forward_patch_method() {
    isahc::send(Request::patch(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .send(Request::patch(stubr.uri()).body(()).unwrap())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "1712620373831678724",
        json!({
            "request": {"method": "PATCH"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/delete.json")]
async fn proxy_should_forward_delete_method() {
    isahc::delete(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .delete(stubr.uri())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "11155613335209754747",
        json!({
            "request": {"method": "DELETE"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/head.json")]
async fn proxy_should_forward_head_method() {
    isahc::head(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .head(stubr.uri())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "5479092281103967096",
        json!({
            "request": {"method": "HEAD"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/options.json")]
async fn proxy_should_forward_options_method() {
    isahc::send(Request::options(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .send(Request::options(stubr.uri()).body(()).unwrap())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "15040751383196264990",
        json!({
            "request": {"method": "OPTIONS"},
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/trace.json")]
async fn proxy_should_forward_trace_method() {
    isahc::send(Request::trace(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .send(Request::trace(stubr.uri()).body(()).unwrap())
        .expect_status_ok();
    assert_recorded_stub_eq(
        "14259663127736384512",
        json!({
            "request": {"method": "TRACE"},
            "response": {"status": 200}
        }),
    )
}
