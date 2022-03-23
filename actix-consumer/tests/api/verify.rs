use actix_web::App;

use actix_consumer::{api::store, repository::store::StoreRepository};
use stubr::*;

use crate::utils::*;

#[actix_web::test]
async fn should_verify() {
    App::new()
        .app_data(fake_store_repository())
        .service(store::find_all)
        .service(store::find_by_id)
        .service(store::create)
        .wrap(ActixVerifyLifecycle::<StoreRepository>(|repo| {
            repo.delete_all()
                .and_then(|_| repo.insert_all(fake_stores()))
                .unwrap()
        }))
        .verify()
        .await;
}