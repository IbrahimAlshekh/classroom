use serde::{Deserialize, Serialize};
use crate::models::class::ClassId;

pub type ScheduleId = u32;

#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
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

    pub fn name(&self) -> &String {
        &self.name
    }

    fn get_class_id(&self) -> ClassId {
        self.class_id
    }
}
