use crate::model::task::Task;
use crate::model::priority::Priority;
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    pub next_id: u32, 
    pub file_name: String,
}

impl TodoList {
    /// Create a new empty todo list
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::<Task>::new(),
            next_id : 1,
            file_name: "todo_list.json".to_string(),
        }
    }

    /// Add a new task to the todo list
    pub fn add_task(&mut self, description:String, priority:Priority) -> String{
        let id_new = format!("{:X}", self.next_id);
        self.tasks.push(Task {
            id : id_new.clone(),
            description,
            priority,
            completed: false
        });
        self.next_id += 1;
        self.save();
        id_new
    }

    /// Return the tasks in the todo list
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Mark a task as completed by ID
    pub fn complete_task(&mut self, id: String) {

        /* Some(T) is part of the Option<T> enum in Rust
        Option<T> can be Some(T) or None and the code below 
        checks if the task with the given ID exists if the 
        expression returned is None then the else block is executed
        */

        // the if let with Option<T> is a way to match against the Some(T) variant
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed; 
            self.save();
        } else {
            panic!("IllegalState Error: Task with ID {} not found when trying to toggle its state.", id);
        }
        
    }

    /// Remove a task from the todo list by ID
    pub fn remove_task(&mut self, id: String) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.save();
        } else {
            panic!("IllegalState Error: Task with ID {} not found when trying to remove.", id);  
        }
    }

    /// Save the todo list into the default JSON file name stored
    pub fn save(&self) {
        self.save_to_file(&self.file_name).expect("Failed to save todo list to the default file");
    }

    /// Save the todo list into a JSON file where the file name is passed as a parameter
    pub fn save_to_file(&self, file_name: &str) -> Result<(), String> {
        let json_data = serde_json::to_string(&self.tasks)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?; 
            
            //The ? operator is used to propagate the posible serialization errors and directly return Err(e)

        std::fs::write(file_name, json_data)
            .map_err(|e| format!("Failed to write to the file {} Err: {}", file_name, e))?;
        
        Ok(())
    }

    pub fn try_load(&mut self) -> Result<(), ()> {
        if !Path::new(&self.file_name).exists() {
            Err(())
        } else {
            self.load();
            Ok(())
        }
    }

    pub fn load(&mut self) {
        let file_name = self.file_name.clone();
        if let Err(_) = self.load_from_file(&file_name) {
            self.save(); 
        }
    }

    //Load the todo list from a JSON file
    pub fn load_from_file(&mut self, file_name: &str) -> Result<(), String> {
        let data = std::fs::read_to_string(file_name)
            .map_err(|e| format!("Failed to read the file '{}'. Err: {}", file_name, e))?;

        let deserialized_tasks : Vec<Task> = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to deserialize tasks: {}", e))?;

        self.tasks = deserialized_tasks;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_new_todo_list() {
        let todo_list = TodoList::new();
        assert_eq!(todo_list.tasks.len(), 0);
        assert_eq!(todo_list.next_id, 1);
        assert_eq!(todo_list.file_name, "todo_list.json");
    }

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".to_string(), Priority::High);
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.next_id, 2);
        assert_eq!(todo_list.tasks[0].id, id);
        assert_eq!(todo_list.tasks[0].description, "Test task");
        assert_eq!(todo_list.tasks[0].priority, Priority::High);
        assert_eq!(todo_list.tasks[0].completed, false);
    }

    #[test]
    fn test_complete_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".to_string(), Priority::Medium);
        todo_list.complete_task(id.clone());
        assert_eq!(todo_list.tasks[0].completed, true);
        // Toggle again to test flipping back
        todo_list.complete_task(id.clone());
        assert_eq!(todo_list.tasks[0].completed, false);
    }

    #[test]
    #[should_panic(expected = "IllegalState Error: Task with ID notfound not found")]
    fn test_complete_task_not_found() {
        let mut todo_list = TodoList::new();
        todo_list.complete_task("notfound".to_string());
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".to_string(), Priority::Low);
        todo_list.remove_task(id.clone());
        assert_eq!(todo_list.tasks.len(), 0);
    }

    #[test]
    #[should_panic(expected = "IllegalState Error: Task with ID notfound not found")]
    fn test_remove_task_not_found() {
        let mut todo_list = TodoList::new();
        todo_list.remove_task("notfound".to_string());
    }

    #[test]
    fn test_save_and_load() {
        let mut todo_list = TodoList::new();
        let test_file = "test_todo_list.json";
        todo_list.file_name = test_file.to_string();

        // Add a task and save
        todo_list.add_task("Test task".to_string(), Priority::High);
        todo_list.save();

        // Load into a new TodoList
        let mut new_todo_list = TodoList::new();
        new_todo_list.file_name = test_file.to_string();
        new_todo_list.load();

        assert_eq!(new_todo_list.tasks.len(), 1);
        assert_eq!(new_todo_list.tasks[0].description, "Test task");
        assert_eq!(new_todo_list.tasks[0].priority, Priority::High);

        // Clean up
        fs::remove_file(test_file).expect("Failed to clean up test file");
    }
}