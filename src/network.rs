use tokio::sync::broadcast;
use uuid::Uuid;
use crate::node::Node;
use crate::{NodeError, NodeState, Task};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;


pub struct Network {
    nodes: Arc<RwLock<HashMap<Uuid, Node>>>,
    task_channel: broadcast::Sender<Task>

}

impl Network {
    pub fn new() -> (Self, broadcast::Receiver<Task>) {
        let (tx, rx) = broadcast::channel(1000);
        (Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            task_channel: tx
        }, rx)
    }

    pub async fn add_node(&self, node: Node) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id, node);
    }

    pub async fn distribute_task(&self, task: Task) -> Result<(), NodeError> {
        let nodes = self.nodes.read().await;


        // Find the most suitable node based on capacity and state
        let suitable_nodes = nodes.values().find(|node| node.state == NodeState::Ready 
            && node.task.len() < node.capacity);


            if let Some(node) = suitable_nodes {
                node.accept_task(task).await?;
                Ok(())
            } else {
                Err(NodeError::TaskDistributionError("No suitable node found".to_string()))
            }
        }

}

