use actix_web::test::{call_service, init_service, TestRequest};
use asserhttp::*;

#[actix_web::test]
async fn record_actix() {
    let app = actix_web::App::new()
        .route("/", actix_web::web::get().to(|| async { actix_web::HttpResponse::Ok().await }))
        // just add this 👇
        .wrap(stubr::ActixRecord::default()); // or `ActixRecord(RecordConfig)` for configuring it
    let req = TestRequest::get().uri("/").to_request();
    let svc = init_service(app).await;
    call_service(&svc, req).await.expect_status_ok();
}
