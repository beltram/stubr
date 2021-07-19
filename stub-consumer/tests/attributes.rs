use asserhttp::*;
use isahc::{AsyncBody, Body};

#[async_std::test]
#[stubr::apps("stubr-producer")]
async fn app_should_run_single_producer() {
    isahc::get_async(stubr_producer.uri()).await.expect_status_ok();
    isahc::post_async(stubr_producer.uri(), AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async(stubr_producer.uri()).await.expect_status_client_error();
}

#[async_std::test]
#[stubr::apps("stubr-producer", "actix-producer")]
async fn app_should_run_many_producer() {
    isahc::get_async(stubr_producer.uri()).await.expect_status_ok();
    isahc::post_async(stubr_producer.uri(), AsyncBody::empty()).await.expect_status_created();
    isahc::delete_async(stubr_producer.uri()).await.expect_status_client_error();
    isahc::get_async(actix_producer.uri()).await.expect_status_ok();
    isahc::delete_async(actix_producer.uri()).await.expect_status_client_error();
}

#[test]
#[stubr::apps("stubr-producer")]
fn blocking_app_should_run_single_producer() {
    isahc::get(stubr_producer.uri()).expect_status_ok();
    isahc::post(stubr_producer.uri(), Body::empty()).expect_status_created();
    isahc::delete(stubr_producer.uri()).expect_status_client_error();
}

#[test]
#[stubr::apps("stubr-producer", "actix-producer")]
fn blocking_app_should_run_many_producers() {
    isahc::get(stubr_producer.uri()).expect_status_ok();
    isahc::post(stubr_producer.uri(), Body::empty()).expect_status_created();
    isahc::delete(stubr_producer.uri()).expect_status_client_error();
    isahc::get(actix_producer.uri()).expect_status_ok();
    isahc::delete(actix_producer.uri()).expect_status_client_error();
}