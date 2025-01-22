use thiserror::Error;


#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task validation failed: {0}")]
    ValidationError(String),

    #[error("Task execution failed: {0}")]
    ExecutionError(String),

    #[error("Task timeout")]
    TimeoutError,

    #[error("Invalid task state transition: {from:?} -> {to:?}")]
    InvalidStateTransition{
        from: String,
        to: String
    }


}