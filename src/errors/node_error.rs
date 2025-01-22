use thiserror::Error;


#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Node Initializing failed: {0}")]
    InitializingString(String),

    #[error("Node Connection failed: {0}")]
    ConnectionError(String),

    #[error("State transition error: {0}")]
    StateError(String),

    #[error("Task Distribution Erorr: {0}")]
    TaskDistributionError(String),

    #[error("Node capacity Exceeded")]
    CapacityExceeded


}