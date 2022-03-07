use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Pet {
    pub id: Option<usize>,
    pub name: String,
}

impl PartialEq for Pet {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Pet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}