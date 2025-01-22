use std::time::Duration;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::states::task_state::TaskState;
use crate::errors::TaskError;
use chrono::{DateTime, Utc}; // Use specific items from the chrono crate
use serde_json::Value;


#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    state: TaskState,
    priority: u8,
    created_at: chrono::DateTime<chrono::Utc>,
    assigned_to: Option<Uuid>,
    data: Value,
    pub max_retries: u8,
    pub timeout_secs: u32,
    
}

impl Task {
    pub fn new(priority: u8, 
        data: Value, max_retries: u8, timeout_secs: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: TaskState::Created,
            priority,
            created_at: Utc::now(),
            assigned_to: None,
            data,
            max_retries,
            timeout_secs,

        }
    }

    pub fn validate(&self) -> Result<(), TaskError> {
        // Check if task has a valid ID
        if self.id.is_nil() {
            return Err(TaskError::ValidationError(
                "Task id cannot be nill".to_string()
            ));
        }

        // Validate priority range (0-255 for u8)
        if self.priority == 0 {
            return Err(TaskError::ValidationError(
                "Priority must be greater than 0".to_string()
            ));
        }

        // Validate creation time
        let hours = 5;
        let now = Utc::now();
        let max_age = Duration::new(hours * 3600, 0);
        if self.created_at + max_age < now {
            return Err(TaskError::ValidationError(
                "Task is too old (>24hr)".to_string()
            ));
        }

        // Validate timeout value
        if self.timeout_secs == 0 || self.timeout_secs > 3600 {
            return Err(TaskError::ValidationError(
                "Timeout between 1 and 3600 seconds".to_string()
            ));

        }

        // Validate max_retries
        if self.max_retries > 5 {
            return Err(TaskError::ValidationError(
                "Max retries cannot exceed 5".to_string()
            ));
        }


        if let Some(node_id) = self.assigned_to {
            if node_id.is_nil() {
                return Err(TaskError::ValidationError(
                    "Assigned node ID cannot be nil".to_string()
                ));
            }

            if self.state != TaskState::Assigned && 
               self.state != TaskState::InProgress && 
               self.state != TaskState::Completed && 
               self.state != TaskState::Failed {
                return Err(TaskError::ValidationError(
                    "Invalid state for assigned task".to_string()
                ));
            }
        }

        Ok(())
    
    }

    pub fn assign_to(&mut self, node_id: Uuid) -> Result<(), TaskError> {
        if self.state != TaskState::Validated {
            return Err(TaskError::InvalidStateTransition { from: format!("{:?}", self.state), 
            to: "Assigned to".to_string()
        });
        }

        self.assigned_to = Some(node_id);
        self.state = TaskState::Assigned;
        Ok(())
    }


}