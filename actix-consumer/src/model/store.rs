use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Store {
    pub id: Option<usize>,
    pub name: String,
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Store {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}