use asserhttp::*;
use isahc::{AsyncBody, Body};

use stubr::Stubr;

#[async_std::test]
async fn app_should_run_producer_stubs() {
    let apps = Stubr::apps(&["stub-producer", "actix-producer"]).await;
    let (simple, actix) = (apps.get(0).unwrap(), apps.get(1).unwrap());
    isahc::get_async(simple.uri()).await.expect_status_ok();
    isahc::post_async(simple.uri(), AsyncBody::empty())
        .await
        .expect_status_created();
    isahc::delete_async(simple.uri()).await.expect_status_client_error();
    isahc::get_async(actix.path("/pets")).await.expect_status_partial_content();
    isahc::delete_async(actix.uri()).await.expect_status_client_error();
}

#[test]
fn blocking_app_should_run_producer_stubs() {
    let apps = Stubr::apps_blocking(&["stub-producer", "actix-producer"]);
    let (simple, actix) = (apps.get(0).unwrap(), apps.get(1).unwrap());
    isahc::get(simple.uri()).expect_status_ok();
    isahc::post(simple.uri(), Body::empty()).expect_status_created();
    isahc::delete(simple.uri()).expect_status_client_error();
    isahc::get(actix.path("/pets")).expect_status_partial_content();
    isahc::delete(actix.uri()).expect_status_client_error();
}
