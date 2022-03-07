use std::{collections::HashSet, sync::Mutex};

use super::super::{error::ApiError, model::pet::Pet};

pub struct PetRepository(Mutex<HashSet<Pet>>);

impl PetRepository {
    pub fn init() -> Self {
        Self(Mutex::new(HashSet::new()))
    }

    pub fn find_all(&self) -> Result<Vec<Pet>, ApiError> {
        self.0.lock()
            .map_err(|_| ApiError::InternalError)
            .map(|db| db.iter().map(|e| e.to_owned()).collect::<Vec<Pet>>())
    }

    pub fn create(&self, mut entity: Pet) -> Result<Pet, ApiError> {
        let id = self.len() + 1;
        self.0.lock()
            .map_err(|_| ApiError::InternalError)
            .and_then(|mut db| {
                entity.id = Some(id);
                if db.insert(entity.clone()) {
                    Ok(entity)
                } else {
                    Err(ApiError::Conflict)
                }
            })
    }

    pub fn insert(&self, entity: Pet) -> Result<(), ApiError> {
        self.0.lock()
            .map_err(|_| ApiError::InternalError)
            .and_then(|mut db| if db.insert(entity) { Ok(()) } else { Err(ApiError::Conflict) })
    }

    pub fn insert_all(&self, entities: Vec<Pet>) -> Result<(), ApiError> {
        for e in entities {
            self.insert(e)?;
        }
        Ok(())
    }

    pub fn delete_all(&self) -> Result<(), ApiError> {
        self.0.lock().as_mut()
            .map_err(|_| ApiError::InternalError)
            .map(|db| db.clear())
    }

    pub fn len(&self) -> usize {
        self.0.lock().as_mut()
            .map_err(|_| ApiError::InternalError)
            .map(|db| db.len())
            .unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Vec<Pet>> for PetRepository {
    fn from(pets: Vec<Pet>) -> Self {
        Self(Mutex::new(HashSet::from_iter(pets)))
    }
}