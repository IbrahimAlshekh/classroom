use crate::models::class::{Class, ClassId};
use crate::models::schedule::{Schedule, ScheduleId};
use crate::models::teacher::{Teacher, TeacherId};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub type SchoolId = i32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct School {
    id: SchoolId,
    name: String,
    teachers: IndexMap<TeacherId, Teacher>,
    classes: IndexMap<ClassId, Class>,
    schedules: IndexMap<ScheduleId, Schedule>,
    next_teacher_id: TeacherId,
    next_class_id: ClassId,
    next_schedule_id: ScheduleId,
}

impl School {
    fn new(id: SchoolId, name: String) -> Self {
        Self {
            id,
            name,
            teachers: IndexMap::new(),
            classes: IndexMap::new(),
            schedules: IndexMap::new(),
            next_teacher_id: 1,
            next_class_id: 1,
            next_schedule_id: 1,
        }
    }

    pub fn id(&self) -> SchoolId {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn get_next_teacher_id(&mut self) -> TeacherId {
        let current_id = self.next_teacher_id;
        self.next_teacher_id += 1;
        current_id
    }

    pub fn get_next_class_id(&mut self) -> ClassId {
        let current_id = self.next_class_id;
        self.next_class_id += 1;
        current_id
    }

    pub fn get_next_schedule_id(&mut self) -> ScheduleId {
        let current_id = self.next_schedule_id;
        self.next_schedule_id += 1;
        current_id
    }

    pub fn next_class_id(&self) -> ClassId {
        self.next_class_id
    }

    fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.insert(teacher.id(), teacher);
    }

    fn get_teacher(&self, id: TeacherId) -> Option<&Teacher> {
        self.teachers.get(&id)
    }

    fn remove_teacher(&mut self, id: TeacherId) {
        self.teachers.shift_remove(&id);
    }

    fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id(), class);
    }

    fn get_class(&self, id: ClassId) -> Option<&Class> {
        self.classes.get(&id)
    }

    fn remove_class(&mut self, id: ClassId) {
        self.classes.shift_remove(&id);
    }

    fn add_schedule(&mut self, schedule: Schedule) {
        self.schedules.insert(schedule.id(), schedule);
    }

    fn get_schedule(&self, id: ScheduleId) -> Option<&Schedule> {
        self.schedules.get(&id)
    }

    fn remove_schedule(&mut self, id: ScheduleId) {
        self.schedules.shift_remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn school_can_be_created() {
        let school = School::new(1, "Test School".to_string());
        assert_eq!(school.id(), 1);
    }

    #[test]
    fn school_can_get_next_teacher_id() {
        let mut school = School::new(1, "Test School".to_string());
        assert_eq!(school.get_next_teacher_id(), 1);
        assert_eq!(school.get_next_teacher_id(), 2);
    }

    #[test]
    fn school_can_get_next_class_id() {
        let mut school = School::new(1, "Test School".to_string());
        assert_eq!(school.get_next_class_id(), 1);
        assert_eq!(school.get_next_class_id(), 2);
    }

    #[test]
    fn school_can_get_next_schedule_id() {
        let mut school = School::new(1, "Test School".to_string());
        assert_eq!(school.get_next_schedule_id(), 1);
        assert_eq!(school.get_next_schedule_id(), 2);
    }

    #[test]
    fn class_can_be_created() {
        let mut school = School::new(1, "Test School".to_string());
        let class_id = school.get_next_class_id();
        let class = Class::new(
            class_id,
            "Test Class".to_string(),
            "Test Description".to_string(),
        );
        let expected_class = class.clone();
        school.add_class(class);
        assert_eq!(school.get_class(class_id), Some(&expected_class));
    }

    #[test]
    fn class_can_be_get_by_id() {
        let mut school = School::new(1, "Test School".to_string());
        let class_id = school.get_next_class_id();
        let class = Class::new(
            class_id,
            "Test Class".to_string(),
            "Test Description".to_string(),
        );
        school.add_class(class.clone());
        assert_eq!(school.get_class(class_id), Some(&class));
    }

    #[test]
    fn class_can_be_removed() {
        let mut school = School::new(1, "Test School".to_string());
        let class_id = school.get_next_class_id();
        let class = Class::new(
            class_id,
            "Test Class".to_string(),
            "Test Description".to_string(),
        );
        school.add_class(class.clone());
        school.remove_class(class_id);
        assert_eq!(school.get_class(class_id), None);
    }

    #[test]
    fn teacher_can_be_created() {
        let mut school = School::new(1, "Test School".to_string());
        let teacher_id = school.get_next_teacher_id();
        let teacher = Teacher::new(
            teacher_id,
            "Test Teacher".to_string(),
            "Test Subject".to_string(),
        );
        let expected_teacher = teacher.clone();
        school.add_teacher(teacher);
        assert_eq!(school.get_teacher(teacher_id), Some(&expected_teacher));
    }

    #[test]
    fn teacher_can_be_get_by_id() {
        let mut school = School::new(1, "Test School".to_string());
        let teacher_id = school.get_next_teacher_id();
        let teacher = Teacher::new(
            teacher_id,
            "Test Teacher".to_string(),
            "Test Subject".to_string(),
        );
        let expected_teacher = teacher.clone();
        school.add_teacher(teacher);
        assert_eq!(school.get_teacher(teacher_id), Some(&expected_teacher));
    }

    #[test]
    fn schedule_can_be_created() {
        let mut school = School::new(1, "Test School".to_string());
        let schedule_id = school.get_next_schedule_id();
        let schedule = Schedule::new(schedule_id, "Test Schedule".to_string(), 1);
        let expected_schedule = schedule.clone();
        school.add_schedule(schedule);
        assert_eq!(school.get_schedule(schedule_id), Some(&expected_schedule));
    }
}
