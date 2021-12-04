use asserhttp::*;
use isahc::{AsyncBody, Body};

use stubr::{Config, Stubr};

#[async_std::test]
async fn app_should_run_producer_stubs() {
    let stubr = Stubr::app("stub-producer").await;
    isahc::get_async(stubr.uri()).await.expect_status_ok();
    isahc::post_async(stubr.uri(), AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async(stubr.uri()).await.expect_status_client_error();
}

#[async_std::test]
async fn app_with_should_run_producer_stubs() {
    let cfg = Config { port: Some(3737), ..Default::default() };
    let _stubr = Stubr::app_with("stub-producer", cfg).await;
    isahc::get_async("http://127.0.0.1:3737").await.expect_status_ok();
    isahc::post_async("http://127.0.0.1:3737", AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async("http://127.0.0.1:3737").await.expect_status_client_error();
}

#[test]
fn blocking_app_should_run_producer_stubs() {
    let stubr = Stubr::app_blocking("stub-producer");
    isahc::get(stubr.uri()).expect_status_ok();
    isahc::post(stubr.uri(), Body::empty()).expect_status_created();
    isahc::delete(stubr.uri()).expect_status_client_error();
}

#[test]
fn blocking_app_with_should_run_producer_stubs() {
    let cfg = Config { port: Some(3737), ..Default::default() };
    let _stubr = Stubr::app_blocking_with("stub-producer", cfg);
    isahc::get("http://127.0.0.1:3737").expect_status_ok();
    isahc::post("http://127.0.0.1:3737", Body::empty()).expect_status_created();
    isahc::delete("http://127.0.0.1:3737").expect_status_client_error();
}
