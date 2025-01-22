use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskState {
    Created,
    Validated,
    Assigned,
    InProgress,
    Completed,
    Failed,
}

