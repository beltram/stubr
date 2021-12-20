use asserhttp::*;
use isahc::Request;
use serde_json::json;

use stubr::*;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/get.json")]
async fn proxy_should_forward_get_method() {
    isahc::get(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(stubr.uri()).expect_status_ok();
    assert_recorded_stub_eq("5416980224536522288", json!({
        "request": {"method": "GET"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/post.json")]
async fn proxy_should_forward_post_method() {
    isahc::post(stubr.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().post(stubr.uri(), ()).expect_status_ok();
    assert_recorded_stub_eq("13490986430150126230", json!({
        "request": {"method": "POST"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/put.json")]
async fn proxy_should_forward_put_method() {
    isahc::put(stubr.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().put(stubr.uri(), ()).expect_status_ok();
    assert_recorded_stub_eq("7811714552500932743", json!({
        "request": {"method": "PUT"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/patch.json")]
async fn proxy_should_forward_patch_method() {
    isahc::send(Request::patch(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::patch(stubr.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("11214217480693164616", json!({
        "request": {"method": "PATCH"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/delete.json")]
async fn proxy_should_forward_delete_method() {
    isahc::delete(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().delete(stubr.uri()).expect_status_ok();
    assert_recorded_stub_eq("2640200797059478006", json!({
        "request": {"method": "DELETE"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/head.json")]
async fn proxy_should_forward_head_method() {
    isahc::head(stubr.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().head(stubr.uri()).expect_status_ok();
    assert_recorded_stub_eq("18176101750997533309", json!({
        "request": {"method": "HEAD"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/options.json")]
async fn proxy_should_forward_options_method() {
    isahc::send(Request::options(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::options(stubr.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("15449147875121094452", json!({
        "request": {"method": "OPTIONS"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/method/trace.json")]
async fn proxy_should_forward_trace_method() {
    isahc::send(Request::trace(stubr.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::trace(stubr.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("4267567086473773488", json!({
        "request": {"method": "TRACE"},
        "response": {"status": 200}
    }))
}
