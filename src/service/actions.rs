use serde::{Deserialize, Serialize};
use crate::model::task::Task;
use crate::model::priority::Priority;
use crate::service::manager::{Manager, ManagerTrait};

/// UndoData enum represents the data needed to undo actions in the todo list.
#[derive(Debug, Clone)]
pub enum UndoRedoData {
    AddTask { task : Task },
    CompleteTask { id: String, previous_state: bool },
    RemoveTask { task : Task },
    EditTask{ previous_task : Task}
}

/// Trait for actions that can be performed on the todo list.
pub trait ActionTrait {
    fn execute(&mut self, manager: &mut Manager) -> UndoRedoData;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddTask { description : String, priority: Priority },
    RemoveTask { task : Task },
    CompleteTask { id: String },
    EditTask { id: String, new_fields : (String, Priority)},
}

impl ActionTrait for Command {
    fn execute(&mut self, manager: &mut Manager) -> UndoRedoData {
        match self {
            Command::AddTask { description, priority } => {
                let id = manager.todo_list.add_task(description.clone(), *priority);
                UndoRedoData::AddTask { 
                    task : Task {id : id, description: description.clone(), priority : *priority, completed: false } 
                }
            },
            Command::RemoveTask { task  } => {
                manager.todo_list.remove_task(task.id.clone());
                UndoRedoData::RemoveTask { task: task.clone() }
            },
            Command::CompleteTask { id } => {
                let previous_state = manager.todo_list.tasks.iter()
                    .find(|task| task.id == *id)
                    .map_or(false, |task| task.completed);
                manager.todo_list.toggle_task_status(id.clone());
                UndoRedoData::CompleteTask { id: id.clone(), previous_state }
            },
            Command::EditTask { id, new_fields } => {
              
                let undo_data = UndoRedoData::EditTask { 
                    previous_task: manager.get_task(id)
                                .expect(&format!("IllegalState Error: Task not found for ID: {}", id))
                                .clone()
                };
                
                manager.todo_list.edit_task(
                    id.as_ref(), 
                    (new_fields.0.as_ref(), &new_fields.1)
                );
                
                undo_data
            },
        }
    }
}



