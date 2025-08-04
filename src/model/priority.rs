use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High
}