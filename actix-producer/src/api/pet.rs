use actix_web::{get, http::StatusCode, post, Responder, web};

use super::super::{error::ApiError, model::pet::Pet, repository::pet::PetRepository};

#[get("/pets")]
pub async fn find_all(db: web::Data<PetRepository>) -> Result<impl Responder, ApiError> {
    db.find_all()
        .map(web::Json)
        .map(|pets| (pets, StatusCode::PARTIAL_CONTENT))
}

#[get("/pets/{id}")]
pub async fn find_by_id(db: web::Data<PetRepository>, path: web::Path<usize>) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    db.find_by_id(id)
        .map(web::Json)
        .map(|pet| (pet, StatusCode::OK))
}

#[post("/pets")]
pub async fn create(pet: web::Json<Pet>, db: web::Data<PetRepository>) -> Result<impl Responder, ApiError> {
    db.create(pet.0)
        .map(web::Json)
        .map(|pet| (pet, StatusCode::CREATED))
}