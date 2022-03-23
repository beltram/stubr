use actix_web::{get, http::StatusCode, post, Responder, web};
use futures::future::join_all;

use super::super::{
    client::pet::PetClient,
    error::ApiError,
    model::{pet::Pet, store::Store},
    repository::store::StoreRepository,
};

#[get("/stores")]
pub async fn find_all(db: web::Data<StoreRepository>) -> Result<impl Responder, ApiError> {
    db.find_all()
        .map(web::Json)
        .map(|stores| (stores, StatusCode::PARTIAL_CONTENT))
}

#[get("/stores/{id}")]
pub async fn find_by_id(db: web::Data<StoreRepository>, path: web::Path<usize>, client: web::Data<PetClient>) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let mut store = db.find_by_id(id)?;
    let pets = find_all_pets(store.pets.iter().filter_map(|p| p.id), client).await;
    store.pets = pets;
    Ok((web::Json(store), StatusCode::OK))
}

#[post("/stores")]
pub async fn create(store: web::Json<Store>, db: web::Data<StoreRepository>, client: web::Data<PetClient>) -> Result<impl Responder, ApiError> {
    let pets = create_all_pets(&store.pets, client).await;
    db.create(store.0.set_pets(pets))
        .map(web::Json)
        .map(|store| (store, StatusCode::CREATED))
}

async fn create_all_pets(pets: &[Pet], client: web::Data<PetClient>) -> Vec<Pet> {
    join_all(pets.iter().map(|p| client.create(p))).await.into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<Pet>>()
}

async fn find_all_pets(ids: impl Iterator<Item=usize>, client: web::Data<PetClient>) -> Vec<Pet> {
    join_all(ids.map(|i| client.find_by_id(i))).await.into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<Pet>>()
}