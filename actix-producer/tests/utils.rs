use actix_web::web;

use actix_producer::{model::pet::Pet, repository::pet::PetRepository};

pub fn fake_pet_repository() -> web::Data<PetRepository> {
    web::Data::new(PetRepository::from(fake_pets()))
}

pub fn empty_pet_repository() -> web::Data<PetRepository> {
    web::Data::new(PetRepository::from(vec![]))
}

pub fn fake_pets() -> Vec<Pet> {
    vec![
        Pet { id: Some(1), name: String::from("rex") },
    ]
}