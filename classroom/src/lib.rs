use crate::models::{School, SchoolId, SubjectId, Teacher};

pub mod models;

struct App {
    school: School,
}

impl App {
    pub fn new(id: SchoolId, name: String) -> Self {
        Self {
            school: models::School::new(id, name),
        }
    }
}
