use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeState {
    Initializing,
    Ready,
    Active,
    Busy,
    Maintenance,
    Disconnected,
}


impl NodeState {

    pub fn can_transition_to(&self, next_state: &NodeState) -> bool {
        match (self, next_state) {
            (NodeState::Initializing, NodeState::Ready) => true,
            (NodeState::Ready, NodeState::Active) => true,
            (NodeState::Active, NodeState::Busy) => true,
            (NodeState::Busy, NodeState::Maintenance) => true,
            (NodeState::Maintenance, NodeState::Ready) => true,
            _ => false,
        }
    }
}