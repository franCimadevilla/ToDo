use serde::{Serialize, Deserialize};
use crate::model::priority::Priority;
use crate::service::manager::{Manager};

/// UndoData enum represents the data needed to undo actions in the todo list.
pub enum UndoData {
    AddTask { id :u32 },
    CompleteTask { id: u32, previous_state: bool },
    RemoveTask { id: u32 },
}

/// Trait for actions that can be performed on the todo list.
pub trait ActionTrait {
    fn execute(&mut self, manager: &mut Manager) -> UndoData;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddTask { description: String, priority: Priority },
    RemoveTask { id: u32 },
    CompleteTask { id: u32 },
}

impl ActionTrait for Command {
    fn execute(&mut self, manager: &mut Manager) -> UndoData {
        match self {
            Command::AddTask { description, priority } => {
                let id = manager.todo_list.add_task(description.clone(), *priority);
                UndoData::AddTask { id }
            },
            Command::RemoveTask { id } => {
                manager.todo_list.remove_task(*id);
                UndoData::RemoveTask { id: *id }
            },
            Command::CompleteTask { id } => {
                let previous_state = manager.todo_list.tasks.iter()
                    .find(|task| task.id == *id)
                    .map_or(false, |task| task.completed);
                manager.todo_list.complete_task(*id);
                UndoData::CompleteTask { id: *id, previous_state }
            },
        }
    }
}



