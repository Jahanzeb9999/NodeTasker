use async_trait::async_trait;
use uuid::Uuid;
use crate::states::node_state::NodeState;
use crate::errors::NodeError;
use crate::task::Task;
use crate::TaskError;
use dashmap::DashMap;
use crate::utils::metrics::NodeMetrics;


pub struct Node {
    pub id: Uuid,
    pub state: NodeState,
    pub capacity: usize,
    pub task: DashMap<Uuid, Task>,
    pub metrics: NodeMetrics,

}


#[async_trait]
pub trait NodeBehavior {
    async fn process_task(&self, task: Task) -> Result<(), NodeError>;
    async fn update_state(&self, new_state: NodeState) -> Result<(), TaskError>;
    async fn health_check(&self) -> bool;
}


impl Node {
    pub fn new(capacity: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: NodeState::Initializing,
            capacity,
            task: DashMap::new(),
            metrics: NodeMetrics::new(),
        }
    }

    pub async fn accept_task(&self, task: Task) -> Result<(), NodeError> {
        if self.task.len() >= self.capacity {
            return Err(NodeError::CapacityExceeded);
        }

        self.task.insert(task.id, task);
        Ok(())
    }


}