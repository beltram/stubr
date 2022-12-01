use actix_web::{App, HttpServer};

use actix_consumer::{api::store, client::pet::PetClient, repository::store::StoreRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(PetClient::default())
            .app_data(StoreRepository::init())
            .service(store::find_all)
            .service(store::find_by_id)
            .service(store::create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
