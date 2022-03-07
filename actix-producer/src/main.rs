use actix_web::*;

use actix_producer::{api::pet, repository::pet::PetRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(PetRepository::init())
            .service(pet::find_all)
            .service(pet::create)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}