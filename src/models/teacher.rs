use serde::{Deserialize, Serialize};
use crate::models::SubjectId;

pub type TeacherId = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Teacher {
    id: TeacherId,
    name: String,
    subject_id: SubjectId,
}

impl Teacher {
    pub fn new(id: TeacherId, name: String, subject_id: SubjectId) -> Self {
        Self {
            id,
            name,
            subject_id,
        }
    }

    pub fn id(&self) -> TeacherId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subject_id(&self) -> &SubjectId {
        &self.subject_id
    }
}
