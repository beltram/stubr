use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn endpoint() -> impl Responder {
    HttpResponse::Ok()
}