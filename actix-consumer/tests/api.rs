use actix_web::{App, test::{call_service, init_service, TestRequest}};
use asserhttp::*;

use actix_consumer::endpoint;

#[actix_rt::test]
async fn should_call_producer() {
    let app = App::new().service(endpoint);
    call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_ok();
}