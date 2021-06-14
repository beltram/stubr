use asserhttp::*;
use isahc::Request;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_get_method() {
    let srv = given("record/method/get");
    isahc::get(srv.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.uri()).expect_status_ok();
    assert_recorded_stub_eq("9217641387224204716", json!({
        "request": {"method": "GET"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_post_method() {
    let srv = given("record/method/post");
    isahc::post(srv.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().post(srv.uri(), ()).expect_status_ok();
    assert_recorded_stub_eq("7695556573860994562", json!({
        "request": {"method": "POST"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_put_method() {
    let srv = given("record/method/put");
    isahc::put(srv.uri(), ()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().put(srv.uri(), ()).expect_status_ok();
    assert_recorded_stub_eq("11293462575583003682", json!({
        "request": {"method": "PUT"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_patch_method() {
    let srv = given("record/method/patch");
    isahc::send(Request::patch(srv.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::patch(srv.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("14869196437315607081", json!({
        "request": {"method": "PATCH"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_delete_method() {
    let srv = given("record/method/delete");
    isahc::delete(srv.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().delete(srv.uri()).expect_status_ok();
    assert_recorded_stub_eq("8005975027299580863", json!({
        "request": {"method": "DELETE"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_head_method() {
    let srv = given("record/method/head");
    isahc::head(srv.uri()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().head(srv.uri()).expect_status_ok();
    assert_recorded_stub_eq("6955969939574470060", json!({
        "request": {"method": "HEAD"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_options_method() {
    let srv = given("record/method/options");
    isahc::send(Request::options(srv.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::options(srv.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("4963237251101014884", json!({
        "request": {"method": "OPTIONS"},
        "response": {"status": 200}
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_trace_method() {
    let srv = given("record/method/trace");
    isahc::send(Request::trace(srv.uri()).body(()).unwrap()).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client().send(Request::trace(srv.uri()).body(()).unwrap()).expect_status_ok();
    assert_recorded_stub_eq("8362021606304740892", json!({
        "request": {"method": "TRACE"},
        "response": {"status": 200}
    }))
}
