use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::models::{ClassId, SubjectId};
use crate::models::week_number::WeekNumber;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct  TimeSlotId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSlot {
    id: TimeSlotId,
    start: DateTime<Local>,
    end: DateTime<Local>,
    week_number: WeekNumber,
    week_day: Weekday,
    subject_id: SubjectId,
    class_id: ClassId,
}