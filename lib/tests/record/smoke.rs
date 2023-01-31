use std::time::Duration;

use asserhttp::*;
use serde_json::json;

use stubr::{RecordConfig, Stubr};

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/smoke/success.json")]
async fn proxy_should_forward_success() {
    isahc::get(stubr.path("/success")).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .get(stubr.path("/success"))
        .expect_status_ok();
    assert_recorded_stub_eq(
        "success-3733506543638807964",
        json!({
            "request": {
                "method": "GET",
                "urlPath": "/success"
            },
            "response": {"status": 200}
        }),
    )
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/smoke/success.json")]
async fn proxy_should_forward_errors() {
    isahc::get(stubr.path("/not-found")).expect_status_not_found();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .get(stubr.path("/not-found"))
        .expect_status_not_found();
    assert_recorded_stub_eq(
        "not-found-1483931038205293293",
        json!({
            "request": {
                "method": "GET",
                "urlPath": "/not-found"
            },
            "response": {"status": 404}
        }),
    )
}

#[cfg(not(target_os = "windows"))]
#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/200.json")]
async fn recorder_should_have_graceful_shutdown() {
    {
        Stubr::record_with(RecordConfig {
            port: Some(1234),
            ..record_cfg()
        })
        .isahc_client()
        .get(stubr.path("/status/200"))
        .expect_status_ok();
    }
    std::thread::sleep(Duration::from_millis(100));
    // <- first recorder should be dropped and socket unbinded
    {
        Stubr::record_with(RecordConfig {
            port: Some(1234),
            ..record_cfg()
        })
        .isahc_client()
        .get(stubr.path("/status/200"))
        .expect_status_ok();
    }
}

#[test]
#[stubr::mock("record/status/200.json")]
fn should_start_recorder_on_provided_runtime() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            Stubr::record().isahc_client().get(stubr.path("/status/200")).expect_status_ok();
        })
}
