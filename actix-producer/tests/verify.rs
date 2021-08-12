use actix_web::App;

use actix_producer::ok;
use stubr::*;

#[actix_rt::test]
async fn should_verify() {
    App::new().service(ok).verify().await;
}