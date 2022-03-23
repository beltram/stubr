use actix_web::{get, http::StatusCode, post, Responder, web};

use super::super::{error::ApiError, model::store::Store, repository::store::StoreRepository};

#[get("/stores")]
pub async fn find_all(db: web::Data<StoreRepository>) -> Result<impl Responder, ApiError> {
    db.find_all()
        .map(web::Json)
        .map(|stores| (stores, StatusCode::PARTIAL_CONTENT))
}

#[get("/stores/{id}")]
pub async fn find_by_id(db: web::Data<StoreRepository>, path: web::Path<usize>) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    db.find_by_id(id)
        .map(web::Json)
        .map(|store| (store, StatusCode::OK))
}

#[post("/stores")]
pub async fn create(store: web::Json<Store>, db: web::Data<StoreRepository>) -> Result<impl Responder, ApiError> {
    db.create(store.0)
        .map(web::Json)
        .map(|store| (store, StatusCode::CREATED))
}