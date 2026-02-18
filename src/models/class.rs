use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use crate::models::SubjectId;

pub type ClassId = u32;

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

#[cfg(test)]
mod class_tests {
    use super::*;

    #[test]
    fn subject_can_be_added_to_class() {
        let mut class = Class::new(1, "Test Class".to_string(), "Test Description".to_string());
        class.add_subject(1);
        assert_eq!(class.subject_ids().len(), 1);
        assert!(class.subject_ids().contains(&1));
    }

    #[test]
    fn subject_can_be_removed_from_class() {
        let mut class = Class::new(1, "Test Class".to_string(), "Test Description".to_string());
        class.add_subject(1);
        class.remove_subject(1);
        assert_eq!(class.subject_ids().len(), 0);
        assert!(!class.subject_ids().contains(&1));
    }
}