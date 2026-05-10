
#[derive( Debug, thiserror::Error )]
pub enum DomainError {
    #[error( "Invalid input" )]
    InvalidInput,

    #[error( "Invalid week number, must be between 1 and 52" )]
    InvalidWeekNumber,
}