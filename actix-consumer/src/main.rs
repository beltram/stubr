use actix_web::{App, HttpServer};

use actix_consumer::endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(endpoint))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}