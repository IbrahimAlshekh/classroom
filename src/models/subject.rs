use crate::models::class::ClassId;
use serde::{Deserialize, Serialize};

pub type SubjectId = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subject {
    id: SubjectId,
    name: String,
}

impl Subject {
    pub fn new(id: SubjectId, name: String, class_id: ClassId) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> SubjectId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
