use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock("record/path/simple.json")]
async fn proxy_should_forward_path() {
    isahc::get(stubr.path("/a/b/c")).expect_status_ok();
    Stubr::record_with(record_cfg())
        .isahc_client()
        .get(stubr.path("/a/b/c"))
        .expect_status_ok();
    assert_recorded_stub_eq(
        "a-b-c-2182426142833608396",
        json!({
            "request": {
                "method": "GET",
                "urlPath": "/a/b/c"
            },
            "response": {"status": 200}
        }),
    )
}
