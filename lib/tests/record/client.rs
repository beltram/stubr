use asserhttp::*;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/client/isahc.json")]
async fn isahc_client_should_proxy_to_recorder() {
    isahc::get(stubr.path("/record/isahc")).expect_status_ok();
    Stubr::record_with(record_cfg()).isahc_client()
        .get(stubr.path("/record/isahc"))
        .expect_status_ok();
    assert_recorded_stub_exists("record-isahc-240440901283558448")
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/client/reqwest.json")]
async fn reqwest_client_should_proxy_to_recorder() {
    reqwest::get(stubr.path("/record/reqwest")).await.expect_status_ok();
    Stubr::record_with(record_cfg()).reqwest_client()
        .get(stubr.path("/record/reqwest")).send().await
        .expect_status_ok();
    assert_recorded_stub_exists("record-reqwest-3255972930125209685")
}