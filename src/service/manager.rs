use crate::model::priority::Priority;
use crate::model::todo_list::TodoList;
use crate::service::actions::{Command, UndoData, ActionTrait};
use crate::model::task::Task;
use crate::ui::displayer_trait::Displayer;

pub struct Manager {
    pub todo_list: TodoList,
    pub undo_stack: Vec<(Command, UndoData)>,
    pub redo_stack: Vec<(Command, UndoData)>,
    pub displayer: Option<Box<dyn Displayer>>,
}

pub trait ManagerTrait {
    fn new(displayer : Box<dyn Displayer>) -> Self;
    fn run(&mut self);
    fn add_task(&mut self, description: String, priority: Priority);
    fn get_tasks(&self) -> &Vec<Task>;
    fn complete_task(&mut self, task_id: String) -> bool;
    fn remove_task(&mut self, task_id: String) -> bool;
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

    fn run(&mut self) {
        if let Some(displayer) = self.displayer.as_ref() {
            if let Err(_) = self.todo_list.try_load() {
                displayer.notify("No previous todo list found... Created a new one🦀");
            }
        }
        
        if let Some(mut displayer) = self.displayer.take() {
            displayer.run(self); 
            self.displayer = Some(displayer); 
        }
    }

    fn add_task(&mut self, description: String, priority: Priority) {
        let mut command = Command::AddTask {
            description,
            priority,
        };
        let undo_data = command.execute(self); 
        self.undo_stack.push((command, undo_data));
        self.redo_stack.clear();
        
    }

    fn get_tasks(&self) -> &Vec<Task> {
        self.todo_list.get_tasks()
    }

    fn complete_task(&mut self, task_id: String) -> bool {
        if self.get_tasks().iter().find(|task| task.id == task_id).is_none() {
            return false;
        } else {
            let mut command = Command::CompleteTask { id: task_id };
            let undo_data = command.execute( self);
            self.undo_stack.push((command, undo_data));
            self.redo_stack.clear();
            return true;
        }
    }

    fn remove_task(&mut self, task_id: String) -> bool {
        if self.get_tasks().iter().find(|t| t.id == task_id).is_none() {
            return false;
        } else {
            let mut command = Command::RemoveTask { id: task_id };
            let undo_data = command.execute(self);
            self.undo_stack.push((command, undo_data));
            self.redo_stack.clear();
            return true;
        }
    }

    fn undo(&mut self) -> Result<bool, String> {
        if let Some((command, undo_data)) = self.undo_stack.pop() {
            
            let redo_undo_data_copy = undo_data.clone();
            match &undo_data {
                UndoData::AddTask { id } => {
                    self.todo_list.remove_task(id.clone());
                },
                UndoData::CompleteTask { id, previous_state } => {
                    if *previous_state {
                        self.todo_list.complete_task(id.clone());
                    } else {
                        self.todo_list.complete_task(id.clone());
                    }
                },
                UndoData::RemoveTask { id } => {
                    self.todo_list.add_task(
                        self.get_tasks().iter()
                            .find(|task| task.id == *id)
                            .map_or("Unknown".to_string(), |task| task.description.clone()),

                        self.get_tasks().iter()
                            .find(|task| task.id == *id)
                            .map_or(Priority::Low, |task| task.priority),
                    );
                },
            }
            self.redo_stack.push((command, redo_undo_data_copy));
            Ok(true)
        } else {
            // No action to undo
            return Ok(false);
        }
    }

    fn redo(&mut self) -> Result<bool, String> {
        if let Some((command, undo_data)) = self.redo_stack.pop() {
            match &command {
                Command::AddTask { description, priority } => {
                    self.todo_list.add_task(description.clone(), priority.clone().clone());
                },
                Command::CompleteTask { id } => {
                    self.todo_list.complete_task(id.clone());
                },
                Command::RemoveTask { id } => {
                    self.todo_list.remove_task(id.clone());
                },
            }
            self.undo_stack.push((command, undo_data));
            Ok(true)
        } else {
            // No action to redo
            return Ok(false); 
        }
    }
}