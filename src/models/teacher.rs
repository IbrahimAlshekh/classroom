use serde::{Deserialize, Serialize};

pub type TeacherId = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Teacher {
    id: TeacherId,
    name: String,
    subject_id: String,
}

impl Teacher {
    pub fn new(id: TeacherId, name: String, subject_id: String) -> Self {
        Self {
            id,
            name,
            subject_id,
        }
    }

    pub fn id(&self) -> TeacherId {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn subject_id(&self) -> &String {
        &self.subject_id
    }
}
