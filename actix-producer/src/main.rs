use actix_web::{App, HttpServer};

use actix_producer::ok;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ok))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}