use crate::model::priority::Priority;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Priority,
    pub completed: bool,
}