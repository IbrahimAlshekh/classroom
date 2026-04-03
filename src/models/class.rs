use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use crate::models::SubjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct  ClassId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    id: ClassId,
    name: String,
    description: String,
    subjects: IndexSet<SubjectId>,
}

impl Class {
    pub fn new(id: ClassId, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            subjects: IndexSet::new(),
        }
    }

    pub fn id(&self) -> ClassId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn subject_ids(&self) -> &IndexSet<SubjectId> {
        &self.subjects
    }

    fn add_subject(&mut self, subject_id: SubjectId) {
        self.subjects.insert(subject_id);
    }

    fn remove_subject(&mut self, subject_id: SubjectId) {
        self.subjects.retain(|id| id != &subject_id);
    }
}