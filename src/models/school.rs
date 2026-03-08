use crate::models::class::{Class, ClassId};
use crate::models::teacher::{Teacher, TeacherId};
use indexmap::{IndexMap, };
use serde::{Deserialize, Serialize};
use crate::models::{Subject, SubjectId, TimeSlot, TimeSlotId};

pub type SchoolId = i32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct School {
    id: SchoolId,
    name: String,
    teachers: IndexMap<TeacherId, Teacher>,
    classes: IndexMap<ClassId, Class>,
    subjects: IndexMap<SubjectId, Subject>,
    timeslots: IndexMap<TimeSlotId, TimeSlot>,
    next_teacher_id: TeacherId,
    next_class_id: ClassId,
    next_timeslot_id: TimeSlotId,
    next_subject_id: SubjectId,
}

impl School {
    pub fn new(id: SchoolId, name: String) -> Self {
        Self {
            id,
            name,
            teachers: IndexMap::new(),
            classes: IndexMap::new(),
            timeslots: IndexMap::new(),
            subjects: IndexMap::new(),
            next_teacher_id: 1,
            next_class_id: 1,
            next_timeslot_id: 1,
            next_subject_id: 1,
        }
    }

    pub fn id(&self) -> SchoolId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn next_teacher_id(&mut self) -> TeacherId {
        let current_id = self.next_teacher_id;
        self.next_teacher_id += 1;
        current_id
    }

    fn next_class_id(&mut self) -> ClassId {
        let current_id = self.next_class_id;
        self.next_class_id += 1;
        current_id
    }

    fn next_timeslot_id(&mut self) -> TimeSlotId {
        let current_id = self.next_timeslot_id;
        self.next_timeslot_id += 1;
        current_id
    }

    pub fn next_subject_id(&mut self) -> SubjectId {
        let current_id = self.next_subject_id;
        self.next_subject_id += 1;
        current_id
    }

    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.insert(teacher.id(), teacher);
    }

    pub fn teacher(&self, id: TeacherId) -> Option<&Teacher> {
        self.teachers.get(&id)
    }

    pub fn remove_teacher(&mut self, id: TeacherId) {
        self.teachers.shift_remove(&id);
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id(), class);
    }

    pub fn class(&self, id: ClassId) -> Option<&Class> {
        self.classes.get(&id)
    }

    pub fn remove_class(&mut self, id: ClassId) {
        self.classes.shift_remove(&id);
    }

    pub fn add_subject(&mut self, subject: Subject) {
        self.subjects.insert(subject.id(), subject);
    }

    pub fn subject(&self, id: SubjectId) -> Option<&Subject> {
        self.subjects.get(&id)
    }

    pub fn remove_subject(&mut self, id: SubjectId) {
        self.subjects.shift_remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_school() -> School {
        School::new(1, "Test School".to_string())
    }

    fn create_class(id: ClassId) -> Class {
        Class::new(id, "Test Class".to_string(), "Test Description".to_string())
    }

    fn create_teacher(id: TeacherId) -> Teacher {
        Teacher::new(id, "Test Teacher".to_string(), 1)
    }

    #[test]
    fn school_can_be_created() {
        let school = create_school();
        assert_eq!(school.id(), 1);
    }

    #[test]
    fn school_can_next_teacher_id() {
        let mut school = create_school();
        assert_eq!(school.next_teacher_id(), 1);
        assert_eq!(school.next_teacher_id(), 2);
    }

    #[test]
    fn school_can_next_class_id() {
        let mut school = create_school();
        assert_eq!(school.next_class_id(), 1);
        assert_eq!(school.next_class_id(), 2);
    }

    #[test]
    fn class_can_be_created() {
        let mut school = create_school();
        let class = create_class(school.next_class_id());
        let expected_class = class.clone();
        school.add_class(class);
        assert_eq!(school.class(expected_class.id()), Some(&expected_class));
    }

    #[test]
    fn class_can_be_retrieved_by_id() {
        let mut school = create_school();
        let class = create_class(school.next_class_id());
        school.add_class(class.clone());
        assert_eq!(school.class(class.id()), Some(&class));
    }

    #[test]
    fn class_can_be_removed() {
        let mut school = create_school();
        let class = create_class(school.next_class_id());
        school.add_class(class.clone());
        school.remove_class(class.id());
        assert_eq!(school.class(class.id()), None);
    }

    #[test]
    fn teacher_can_be_created() {
        let mut school = create_school();
        let teacher = create_teacher(school.next_teacher_id());
        let expected_teacher = teacher.clone();
        school.add_teacher(teacher);
        assert_eq!(
            school.teacher(expected_teacher.id()),
            Some(&expected_teacher)
        );
    }

    #[test]
    fn teacher_can_be_retrieved_by_id() {
        let mut school = create_school();
        let teacher = create_teacher(school.next_teacher_id());
        let expected_teacher = teacher.clone();
        school.add_teacher(teacher);
        assert_eq!(
            school.teacher(expected_teacher.id()),
            Some(&expected_teacher)
        );
    }

    #[test]
    fn teacher_can_be_removed() {
        let mut school = create_school();
        let teacher = create_teacher(school.next_teacher_id());
        school.add_teacher(teacher.clone());
        school.remove_teacher(teacher.id());
        assert_eq!(school.teacher(teacher.id()), None);
    }
}
