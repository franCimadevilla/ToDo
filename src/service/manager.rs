use crate::model::priority::Priority;
use crate::model::todo_list::TodoList;
use crate::service::actions::{Command, UndoRedoData, ActionTrait};
use crate::model::task::Task;
use crate::ui::displayer_trait::Displayer;

pub struct Manager {
    pub todo_list: TodoList,
    pub undo_stack: Vec<(Command, UndoRedoData)>,
    pub redo_stack: Vec<(Command, UndoRedoData)>,
    pub displayer: Option<Box<dyn Displayer>>,
}

pub trait ManagerTrait {
    fn new(displayer : Box<dyn Displayer>) -> Self;
    fn run(&mut self);
    fn add_task(&mut self, description: &str, priority: &Priority);
    fn get_tasks(&self) -> &Vec<Task>;
    fn get_task(&self, id: &str) -> Option<&Task>;
    fn get_task_mut(&mut self, id : &str) -> Option<&mut Task>;
    fn toggle_task_status(&mut self, task_id: &str) -> bool;
    fn remove_task(&mut self, task_id: &str) -> bool;
    fn edit_task(&mut self, task_id : &str, new_description : &str, new_priority : &Priority) -> bool;
    fn undo(&mut self) -> Result<bool, String>;
    fn redo(&mut self) -> Result<bool, String>;
}

impl ManagerTrait for Manager {

    /// Creates a new Manager instance with an empty TodoList and empty undo/redo stacks.
    fn new(displayer : Box<dyn Displayer>) -> Self {
        Manager {
            todo_list: TodoList::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            displayer: Some(displayer),
        }
    }

    /// Runs the displayer, loading the todo list if it exists.
    /// If the todo list does not exist, it notifies the user and creates a new one.
    fn run(&mut self) {
        if let Some(displayer) = self.displayer.as_mut() {
            if let Err(_) = self.todo_list.try_load() {
                let _ = displayer.notify("No previous todo list found... Created a new one🦀");
            }
        }
        
        if let Some(mut displayer) = self.displayer.take() {
            displayer.run(self); 
            self.displayer = Some(displayer); 
        }
    }

    /// Adds a new task to the todo list and updates the undo stack.
    /// Clears the redo stack after adding a new task.
    fn add_task(&mut self, description: &str, priority: &Priority) {
        let mut command = Command::AddTask {
            description: description.to_string(),
            priority: *priority,
        };
        let undo_data = command.execute(self); 
        self.undo_stack.push((command, undo_data));
        self.redo_stack.clear();
        
    }

    /// Returns the tasks in the todo list.
    fn get_tasks(&self) -> &Vec<Task> {
        self.todo_list.get_tasks()
    }

    /// Get a task by ID
    fn get_task(&self, id: &str) -> Option<&Task> {
        self.todo_list.get_tasks().iter().find(|task| task.id == id)
    }

    /// Get a mutable reference to a Task by its ID
    fn get_task_mut(&mut self, id : &str) -> Option<&mut Task> {
        self.todo_list.get_task_mut(id)
    }
   
    /// Complete/Uncomplete a task by ID
    /// Returns true if the task was found and toggled, false otherwise.
    fn toggle_task_status(&mut self, task_id: &str) -> bool {
        if self.get_task(task_id).is_none() {
            return false;
        } else {
            let mut command = Command::CompleteTask { id: task_id.into() };
            let undo_data = command.execute( self);
            self.undo_stack.push((command, undo_data));
            self.redo_stack.clear();
            return true;
        }
    }

    /// Remove a task from the todo list by ID
    /// Returns true if the task was found and removed, false otherwise.
    fn remove_task(&mut self, task_id: &str) -> bool {
        let task_ = self.get_task(task_id);

        if task_.is_none() {
            return false;
        } else {
            let mut command = Command::RemoveTask { 
                task : task_.expect("Error when extracting the task during remove").clone() 
            };
            let undo_data = command.execute(self);
            self.undo_stack.push((command, undo_data));
            self.redo_stack.clear();
            return true;
        }
    }

    fn edit_task(&mut self, task_id: &str, new_description : &str, new_priority : &Priority) -> bool{
        let task  = self.get_task_mut(task_id);

        if task.is_none() {
            return false;
        } else {
            let mut command = Command::EditTask { 
                id: task_id.into(),
                new_fields : (new_description.into(), *new_priority)
            };
            let undo_data = command.execute(self);
            self.undo_stack.push((command, undo_data));
            self.redo_stack.clear();
            return true;
        }
    }

    /// Undo the last action performed on the todo list.
    /// Returns Ok(true) if the action was successfully undone, Ok(false) if there was nothing to undo.
    /// Returns an error if the undo operation fails.
    fn undo(&mut self) -> Result<bool, String> {
        if let Some((command, undo_data)) = self.undo_stack.pop() {
            
            let redo_undo_data_copy = undo_data.clone();
            match &undo_data {
                UndoRedoData::AddTask { task } => {
                    self.todo_list.remove_task(task.id.clone());
                },
                UndoRedoData::CompleteTask { id, previous_state } => {
                    if *previous_state {
                        self.todo_list.toggle_task_status(id.clone());
                    } else {
                        self.todo_list.toggle_task_status(id.clone());
                    }
                },
                UndoRedoData::RemoveTask { task } => {
                    self.todo_list.push_task(task.clone());
                },
                UndoRedoData::EditTask { previous_task} => {
                    self.todo_list.edit_task(
                        previous_task.id.as_ref(),
                         (previous_task.description.as_ref(), &previous_task.priority)
                        );
                }
            }
            self.redo_stack.push((command, redo_undo_data_copy));
            Ok(true)
        } else {
            return Ok(false);
        }
    }

    /// Redo the last action performed on the todo list.
    /// Returns Ok(true) if the action was successfully redone, Ok(false) if there was nothing to redo.
    /// Returns an error if the redo operation fails.
    fn redo(&mut self) -> Result<bool, String> {
        if let Some((command, undo_data)) = self.redo_stack.pop() {
            match &command {
                Command::AddTask { description, priority } => {
                    self.todo_list.add_task(description.clone(), priority.clone().clone());
                },
                Command::CompleteTask { id } => {
                    self.todo_list.toggle_task_status(id.clone());
                },
                Command::RemoveTask { task } => {
                    self.todo_list.remove_task(task.id.clone());
                },
                Command::EditTask { id , new_fields } => {
                    self.todo_list.edit_task(
                        id, 
                        (new_fields.0.as_ref(), &new_fields.1)
                    );
                }
            }
            self.undo_stack.push((command, undo_data));
            Ok(true)
        } else {
            return Ok(false); 
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::console_ui::menu_option::MenuOption;

    // Mock Displayer para pruebas
    struct MockDisplayer;
    impl Displayer for MockDisplayer {
        fn new() -> Self { MockDisplayer }
        fn run(&mut self, _manager: &mut Manager) {}
        fn display(&mut self) -> Result<MenuOption, String> { Ok(MenuOption::Exit) }
        fn notify(&mut self, _message: &str) -> Result<(), String> { Ok(()) }
        fn exit(&mut self) -> Result<(), String> { Ok(())}
    }

    #[test]
    fn test_new_manager() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let manager = Manager::new(displayer);
        assert_eq!(manager.todo_list.tasks.len(), 0);
        assert_eq!(manager.undo_stack.len(), 0);
        assert_eq!(manager.redo_stack.len(), 0);
        assert!(manager.displayer.is_some());
    }

    #[test]
    fn test_add_task() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        manager.add_task("Test task".as_ref(), &Priority::High);
        assert_eq!(manager.todo_list.tasks.len(), 1);
        assert_eq!(manager.undo_stack.len(), 1);
        assert_eq!(manager.redo_stack.len(), 0);
        assert_eq!(manager.todo_list.tasks[0].description, "Test task");
        assert_eq!(manager.todo_list.tasks[0].priority, Priority::High);
    }

    #[test]
    fn test_complete_task() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let id = manager.todo_list.add_task("Test task".to_string(), Priority::Medium);
        let result = manager.toggle_task_status(id.as_ref());
        assert!(result);
        assert_eq!(manager.todo_list.tasks[0].completed, true);
        assert_eq!(manager.undo_stack.len(), 1);
        assert_eq!(manager.redo_stack.len(), 0);
    }

    #[test]
    fn test_complete_task_not_found() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let result = manager.toggle_task_status("notfound".as_ref());
        assert!(!result);
        assert_eq!(manager.undo_stack.len(), 0);
    }

    #[test]
    fn test_remove_task() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let id = manager.todo_list.add_task("Test task".to_string(), Priority::Low);
        let result = manager.remove_task(id.as_ref());
        assert!(result);
        assert_eq!(manager.todo_list.tasks.len(), 0);
        assert_eq!(manager.undo_stack.len(), 1);
        assert_eq!(manager.redo_stack.len(), 0);
    }

    #[test]
    fn test_undo_add_task() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        manager.add_task("Test task".as_ref(), &Priority::High);
        let result = manager.undo().expect("Undo failed");
        assert!(result);
        assert_eq!(manager.todo_list.tasks.len(), 0);
        assert_eq!(manager.undo_stack.len(), 0);
        assert_eq!(manager.redo_stack.len(), 1);
    }

    #[test]
    fn test_redo_add_task() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        manager.add_task("Test task".as_ref(), &Priority::High);
        manager.undo().expect("Undo failed");
        let result = manager.redo().expect("Redo failed");
        assert!(result);
        assert_eq!(manager.todo_list.tasks.len(), 1);
        assert_eq!(manager.undo_stack.len(), 1);
        assert_eq!(manager.redo_stack.len(), 0);
    }

    #[test]
    fn test_undo_empty() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let result = manager.undo().expect("Undo failed");
        assert!(!result);
    }

    #[test]
    fn test_undo_empty_stack() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let result = manager.undo();
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_redo_empty_stack() {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer::new());
        let mut manager = Manager::new(displayer);
        let result = manager.redo();
        assert_eq!(result, Ok(false));
    }
}