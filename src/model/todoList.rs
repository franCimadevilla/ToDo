use crate::model::task::Task;
use crate::model::priority::Priority;
use serde_json::{to_string, from_str};
use serde::{Serialize, Deserialize};
use std::fs::{read_to_string, write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo_list {
    tasks: Vec<Task>
}

impl Todo_list {
    /// Create a new empty todo list
    pub fn new() -> Self {
        Todo_list {
            tasks: Vec<Task>::new()
        }
    }

    /// Add a new task to the todo list
    pub fn add_task(&mut self, description:String, priority:Priority) {
        self.tasks.push(Task {
            id: self.tasks.len() as i32 + 1, // Simple ID generation
            description,
            priority,
            completed: false
        });
    }

    /// Shows all the tasks in the todo list
    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("ID: {}, Description: {}, Priority: {:?}, Completed: {}", 
                     task.id, task.description, task.priority, task.completed);  //TODO: Extract prints to a different layer
        }
    }

    /// Mark a task as completed by ID
    pub fn complete_task(&mut self, id: u32) {

        /* Some(T) is part of the Option<T> enum in Rust
        Option<T> can be Some(T) or None and the code below 
        checks if the task with the given ID exists if the 
        expression returned is None then the else block is executed
        */

        // the if let with Option<T> is a way to match against the Some(T) variant
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task with ID {} marked as completed.", id)    //TODO: Extract prints to a different layer
        } else {
            println!("Task with ID {} not found.", id);
        }
        
    }

    /// Remove a task from the todo list by ID
    pub fn remove_task(&mut self, id: u32) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("Task with ID {} removed.", id);  //TODO: Extract prints to a different layer
        } else {
            println!("Task with ID {} not found.", id);  
        }
    }

    /// Save the todo list into a JSON file
    pub fn save_to_file(&self, file_name: &str) -> Result<(), String> {
        let json_data = serde_json::to_string(&self.tasks)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?; 
            
            //The ? operator is used to propagate the posible serialization errors and directly return Err(e)

        std::fs::write(file_name, json_data)
            .map_err(|e| format!("Failed to write to the file {} Err: {}", file_name, e))?;
        
        println!("Todo list susccessfully saved to the file '{}'", file_name); //TODO: Extract to a different layer
        Ok(())
    }

    //Load the todo list from a JSON file
    pub fn load_from_file(&mut self, file_name: &str) -> Result<(), String> {
        let data = std::fs::read_to_string(file_name)
            .map_err(|e| format!("Failed to read the file '{}'. Err: {}", file_name, e))?;

        let deserialized_tasks : Vec<Task> = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to deserialize tasks: {}", e))?;

        self.tasks = deserialized_tasks;
        
        println!("Todo list successfully loaded from the file '{}'", file_name); //TODO: Extract to a different layer
        Ok(())
    }
}