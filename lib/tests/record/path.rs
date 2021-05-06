use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_path() {
    let srv = given("record/path/simple");
    isahc::get(srv.path("/a/b/c")).unwrap().assert_ok();
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/a/b/c")).unwrap().assert_ok();
    assert_recorded_stub_eq("a-b-c-16081596189452964389", json!({
        "request": {
            "method": "GET",
            "urlPath": "/a/b/c"
        },
        "response": {"status": 200}
    }))
}
