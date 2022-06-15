use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::pet::Pet;

#[derive(Serialize, Deserialize, Debug, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Store {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    pub name: String,
    #[serde(default)]
    pub pets: Vec<Pet>,
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Store {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}