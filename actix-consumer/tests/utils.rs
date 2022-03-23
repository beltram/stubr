use actix_web::web;

use actix_consumer::{model::store::Store, repository::store::StoreRepository};

pub fn fake_store_repository() -> web::Data<StoreRepository> {
    web::Data::new(StoreRepository::from(fake_stores()))
}

pub fn empty_store_repository() -> web::Data<StoreRepository> {
    web::Data::new(StoreRepository::from(vec![]))
}

pub fn fake_stores() -> Vec<Store> {
    vec![
        Store { id: Some(1), name: String::from("jardiland") },
    ]
}