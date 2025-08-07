use serde::{Serialize, Deserialize};
use crate::model::priority::Priority;
use crate::service::manager::{Manager};

/// UndoData enum represents the data needed to undo actions in the todo list.
#[derive(Debug, Clone)]
pub enum UndoData {
    AddTask { id :String },
    CompleteTask { id: String, previous_state: bool },
    RemoveTask { id: String },
}

/// Trait for actions that can be performed on the todo list.
pub trait ActionTrait {
    fn execute(&mut self, manager: &mut Manager) -> UndoData;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddTask { description: String, priority: Priority },
    RemoveTask { id: String },
    CompleteTask { id: String },
}

impl ActionTrait for Command {
    fn execute(&mut self, manager: &mut Manager) -> UndoData {
        match self {
            Command::AddTask { description, priority } => {
                let id = manager.todo_list.add_task(description.clone(), *priority);
                UndoData::AddTask { id }
            },
            Command::RemoveTask { id } => {
                manager.todo_list.remove_task(id.clone());
                UndoData::RemoveTask { id: id.clone() }
            },
            Command::CompleteTask { id } => {
                let previous_state = manager.todo_list.tasks.iter()
                    .find(|task| task.id == *id)
                    .map_or(false, |task| task.completed);
                manager.todo_list.complete_task(id.clone());
                UndoData::CompleteTask { id: id.clone(), previous_state }
            },
        }
    }
}



