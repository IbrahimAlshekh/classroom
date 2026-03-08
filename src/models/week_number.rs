use chrono::{DateTime, Datelike, Local};
use serde::{Deserialize, Serialize};
use crate::models::error::DomainError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeekNumber(u8);

impl WeekNumber {
    pub fn new(week_number: u8) -> Result<Self, DomainError> {
        if(week_number < 1 || week_number > 52){
            return Err(DomainError::InvalidWeekNumber);
        }
        Ok(Self(week_number))
    }

    pub fn from_date(date: DateTime<Local>) -> Result<Self, DomainError> {
        let week_number = date.iso_week().week() as u8;
        Ok(Self(week_number))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}