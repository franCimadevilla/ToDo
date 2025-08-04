use crate::model::priority::Priority;
use crate::model::todo_list::TodoList;
use crate::service::actions::{Command, UndoData, ActionTrait};
use crate::model::task::Task;

pub struct Manager {
    pub todo_list: TodoList,
    pub undo_stack: Vec<(Command, UndoData)>,
    pub redo_stack: Vec<(Command, UndoData)>,
}

pub trait ManagerTrait {
    fn new() -> Self;
    fn add_task(&mut self, description: String, priority: Priority);
    fn get_tasks(&self) -> &Vec<Task>;
    fn complete_task(&mut self, task_id: u32);
    fn remove_task(&mut self, task_id: u32);
    fn undo(&mut self);
    fn redo(&mut self);
}

impl ManagerTrait for Manager {

    /// Creates a new Manager instance with an empty TodoList and empty undo/redo stacks.
    fn new() -> Self {
        Manager {
            todo_list: TodoList::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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

    fn complete_task(&mut self, task_id: u32) {
        let mut command = Command::CompleteTask { id: task_id };
        let undo_data = command.execute( self);
        self.undo_stack.push((command, undo_data));
        self.redo_stack.clear();
    }

    fn remove_task(&mut self, task_id: u32) {
        let mut command = Command::RemoveTask { id: task_id };
        let undo_data = command.execute(self);
        self.undo_stack.push((command, undo_data));
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some((command, undo_data)) = self.undo_stack.pop() {
            match undo_data {
                UndoData::AddTask { id } => {
                    self.todo_list.remove_task(id);
                },
                UndoData::CompleteTask { id, previous_state } => {
                    if previous_state {
                        self.todo_list.complete_task(id);
                    } else {
                        self.todo_list.complete_task(id);
                    }
                },
                UndoData::RemoveTask { id } => {
                    self.todo_list.add_task(
                        self.get_tasks().iter()
                            .find(|task| task.id == id)
                            .map_or("Unknown".to_string(), |task| task.description.clone()),

                        self.get_tasks().iter()
                            .find(|task| task.id == id)
                            .map_or(Priority::Low, |task| task.priority),
                    );
                },
            }
            self.redo_stack.push((command, undo_data));
        }
    }

    fn redo(&mut self) {
        if let Some((command, undo_data)) = self.redo_stack.pop() {
            match &command {
                Command::AddTask { description, priority } => {
                    self.todo_list.add_task(description.clone(), priority.clone().clone());
                },
                Command::CompleteTask { id } => {
                    self.todo_list.complete_task(*id);
                },
                Command::RemoveTask { id } => {
                    self.todo_list.remove_task(*id);
                },
            }
            self.undo_stack.push((command, undo_data));
        }
    }
}