use surf::{get, Response};

use stubr::Config;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_path() {
    let srv = given("resp/template/url/path");
    get(&srv.path_query("/api/path", "name", "beltram")).await.unwrap()
        .assert_ok()
        .assert_body_text("/api/path")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_request_url() {
    let srv = given("resp/template/url/url");
    get(&srv.path_query("/api/path", "name", "beltram")).await.unwrap()
        .assert_ok()
        .assert_body_text("/api/path?name&#x3D;beltram")
        .assert_content_type_text();
}

#[async_std::test]
#[ignore] // need to fix this in wiremock-rs
async fn should_template_request_port() {
    let cfg = Config { port: Some(59_000), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/resp/template/url/port.json", cfg).await;
    get(&srv.path("/api/port")).await.unwrap()
        .assert_ok()
        .assert_body_text("59000")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_request_method() {
    let srv = given("resp/template/url/method");
    let expect = |mut response: Response, body: &str| {
        response.assert_ok().assert_body_text(body).assert_content_type_text();
    };
    expect(surf::get(&srv.uri()).await.unwrap(), "GET");
    expect(surf::post(&srv.uri()).await.unwrap(), "POST");
    expect(surf::put(&srv.uri()).await.unwrap(), "PUT");
    expect(surf::delete(&srv.uri()).await.unwrap(), "DELETE");
    expect(surf::patch(&srv.uri()).await.unwrap(), "PATCH");
    expect(surf::options(&srv.uri()).await.unwrap(), "OPTIONS");
    expect(surf::trace(&srv.uri()).await.unwrap(), "TRACE");
}

#[async_std::test]
async fn should_template_request_path_segments() {
    let srv = given("resp/template/url/path-segments");
    get(&srv.path("/one/two/three")).await.unwrap()
        .assert_ok()
        .assert_body_text("two")
        .assert_content_type_text();
}
