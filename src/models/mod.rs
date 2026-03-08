pub mod class;
pub mod school;
pub mod subject;
pub mod teacher;
pub mod timeslot;
mod week_number;
mod error;

// Re-export commonly used types
pub use class::{Class, ClassId};
pub use school::{School, SchoolId};
pub use subject::{Subject, SubjectId};
pub use teacher::{Teacher, TeacherId};
pub use timeslot::{TimeSlot, TimeSlotId};
