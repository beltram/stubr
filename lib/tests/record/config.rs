use asserhttp::*;
use tempfile::tempdir;

use stubr::{RecordConfig, Stubr};

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/200.json")]
async fn should_allow_custom_output() {
    let output = tempdir().unwrap().into_path();
    let cfg = RecordConfig {
        output: Some(output.clone()),
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    Stubr::record_with(cfg).isahc_client().get(stubr.path("/status/200")).expect_status_ok();
    assert!(output.join("localhost").join("status-200-2265763564283130440.json").exists())
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/status/200.json")]
async fn should_allow_recording_on_dedicated_port() {
    let recorder = Stubr::record_with(RecordConfig { port: Some(1234), ..record_cfg() });
    recorder.isahc_client().get(stubr.path("/status/200")).expect_status_ok();
    assert_eq!(recorder.uri(), String::from("http://127.0.0.1:1234"))
}