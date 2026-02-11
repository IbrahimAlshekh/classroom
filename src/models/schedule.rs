use crate::models::class::ClassId;
use serde::{Deserialize, Serialize};

pub type ScheduleId = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    id: ScheduleId,
    name: String,
    class_id: ClassId,
}

impl Schedule {
    pub fn new(id: ScheduleId, name: String, class_id: ClassId) -> Self {
        Self { id, name, class_id }
    }

    pub fn id(&self) -> ScheduleId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class_id(&self) -> ClassId {
        self.class_id
    }
}
