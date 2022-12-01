use super::super::{error::ApiError, model::pet::Pet};

#[derive(Default)]
pub struct PetClient {
    pub client: reqwest::Client,
    pub uri: String,
}

impl PetClient {
    pub async fn create(&self, pet: &Pet) -> Result<Pet, ApiError> {
        Ok(self
            .client
            .post(format!("{}/pets", self.uri))
            .json(pet)
            .header("content-type", "application/json")
            .send()
            .await?
            .json::<Pet>()
            .await?)
    }

    pub async fn find_all(&self) -> Result<Vec<Pet>, ApiError> {
        Ok(self
            .client
            .get(format!("{}/pets", self.uri))
            .send()
            .await?
            .json::<Vec<Pet>>()
            .await?)
    }

    pub async fn find_by_id(&self, id: usize) -> Result<Pet, ApiError> {
        Ok(self
            .client
            .get(format!("{}/pets/{}", self.uri, id))
            .send()
            .await?
            .json::<Pet>()
            .await?)
    }
}

impl From<String> for PetClient {
    fn from(uri: String) -> Self {
        Self { uri, ..Default::default() }
    }
}
