use asserhttp::*;
use tempfile::tempdir;

use stubr::{RecordConfig, Stubr};

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn should_allow_custom_output() {
    let srv = given("record/status/200");
    let output = tempdir().unwrap().into_path();
    let cfg = RecordConfig {
        output: Some(output.clone()),
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    };
    Stubr::record_with(cfg).isahc_client().get(srv.path("/status/200")).expect_status_ok();
    assert!(output.join("localhost").join("status-200-1330526116653087821.json").exists())
}