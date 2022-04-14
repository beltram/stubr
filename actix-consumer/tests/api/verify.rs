use actix_web::{App, web};

use actix_consumer::{api::store, client::pet::PetClient, repository::store::StoreRepository};
use stubr::*;

use crate::utils::*;

// TODO: figure out why fails on CI
// #[actix_web::test]
#[stubr::apps("actix-producer")]
async fn should_verify() {
    App::new()
        .app_data(web::Data::new(PetClient::from(actix_producer.uri())))
        .app_data(fake_store_repository())
        .service(store::find_all)
        .service(store::find_by_id)
        .service(store::create)
        .wrap(ActixVerifyLifecycle::<StoreRepository>(|repo| {
            repo.delete_all()
                .and_then(|_| repo.insert_all(fake_stores()))
                .unwrap()
        }))
        .verify_except(|stub| stub == "cannot-succeed")
        .await;
}