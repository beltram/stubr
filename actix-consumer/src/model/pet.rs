use std::hash::{Hash, Hasher};

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Pet {
    #[serde(skip_serializing_if = "Option::is_none")]
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
