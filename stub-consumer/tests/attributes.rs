use asserhttp::*;
use isahc::{AsyncBody, Body};

#[async_std::test]
#[stubr::apps("stub-producer")]
async fn app_should_run_single_producer() {
    isahc::get_async(stub_producer.uri()).await.expect_status_ok();
    isahc::post_async(stub_producer.uri(), AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async(stub_producer.uri()).await.expect_status_client_error();
}

#[async_std::test]
#[stubr::apps("stub-producer", "actix-producer")]
async fn app_should_run_many_producer() {
    isahc::get_async(stub_producer.uri()).await.expect_status_ok();
    isahc::post_async(stub_producer.uri(), AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async(stub_producer.uri()).await.expect_status_client_error();
    isahc::get_async(actix_producer.path("/pets")).await.expect_status_partial_content();
    isahc::delete_async(actix_producer.uri()).await.expect_status_client_error();
}

#[test]
#[stubr::apps("stub-producer")]
fn blocking_app_should_run_single_producer() {
    isahc::get(stub_producer.uri()).expect_status_ok();
    isahc::post(stub_producer.uri(), Body::empty()).expect_status_created();
    isahc::delete(stub_producer.uri()).expect_status_client_error();
}

#[test]
#[stubr::apps("stub-producer", "actix-producer")]
fn blocking_app_should_run_many_producers() {
    isahc::get(stub_producer.uri()).expect_status_ok();
    isahc::post(stub_producer.uri(), Body::empty()).expect_status_created();
    isahc::delete(stub_producer.uri()).expect_status_client_error();
    isahc::get(actix_producer.path("/pets")).expect_status_partial_content();
    isahc::delete(actix_producer.uri()).expect_status_client_error();
}