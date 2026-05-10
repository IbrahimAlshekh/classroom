use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use crate::models::TeacherId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct  SubjectId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subject {
    id: SubjectId,
    name: String,
    teacher_id: TeacherId,
    subteacher_ids: Option<IndexSet<TeacherId>>,
}

impl Subject {
    pub fn new(id: SubjectId, name: String, teacher_id: TeacherId) -> Self {
        Self {
            id,
            name,
            teacher_id,
            subteacher_ids: None
        }
    }

    pub fn id(&self) -> SubjectId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn teacher_id(&self) -> TeacherId {
        self.teacher_id
    }

    pub fn subteacher_ids(&self) -> Option<&IndexSet<TeacherId>> {
        self.subteacher_ids.as_ref()
    }

    pub fn add_subteacher_id(&mut self, teacher_id: TeacherId) {
        if self.subteacher_ids.is_none() {
            self.subteacher_ids = Some(IndexSet::new());
        }
        self.subteacher_ids.as_mut().unwrap().insert(teacher_id);
    }
}
