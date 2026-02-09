use serde::{Deserialize, Serialize};

pub type ClassId = u32;

#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
pub struct Class {
    id: ClassId,
    name: String,
    description: String
}

impl Class {
    pub fn new(id: ClassId, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description
        }
    }

    pub fn id(&self) -> ClassId {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}