use thiserror::Error;

// Application-wide error types
#[derive(Error, Debug)]
pub enum AppError {
    // todo, add error variants for:
    // database errors (wrap sqlx::Error)
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    // not found errors
    #[error("{0} not found")]
    NotFound(String),
    // validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),
    // duplicate entry errors
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
}

// type alias for Results using AppError
pub type AppResult<T> = Result<T, AppError>;