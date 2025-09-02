use crate::model::priority::Priority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub priority: Priority,
    pub completed: bool,
}
