use crate::model::priority::Priority;
use crate::model::task::Task;
use serde::{Deserialize, Serialize};
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
            next_id: 1,
            file_name: "todo_list.json".to_string(),
        }
    }

    /// Add a new task to the todo list
    pub fn add_task(&mut self, description: String, priority: Priority) -> String {
        let id_new = format!("{:X}", self.next_id);
        self.tasks.push(Task {
            id: id_new.clone(),
            description,
            priority,
            completed: false,
        });
        self.next_id += 1;
        self.save();
        id_new
    }

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push(task);
        self.save();
    }

    /// Return the tasks in the todo list
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id.eq(id))
    }

    /// Mark a task as completed/uncompleted by ID
    pub fn toggle_task_status(&mut self, id: String) {
        // the if let with Option<T> is a way to match against the Some(T) variant
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            self.save();
        } else {
            panic!(
                "IllegalArgument Error: Task with ID: {} not found when trying to toggle its state.",
                id
            );
        }
    }

    /// Remove a task from the todo list by ID
    pub fn remove_task(&mut self, id: String) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.save();
        } else {
            panic!(
                "IllegalArgument Error: Task with ID: {} not found when trying to remove.",
                id
            );
        }
    }

    /// Edit a task fields
    pub fn edit_task(&mut self, id: &str, new_fields: (&str, &Priority)) {
        let task = self.get_task_mut(id).expect(&format!(
            "IllegalState Error: Task not found for ID: {}",
            id
        ));
        if !task.description.eq(new_fields.0) {
            task.description = new_fields.0.to_string()
        }

        if !task.priority.eq(new_fields.1) {
            task.priority = *new_fields.1
        }

        self.save();
    }

    /// Save the todo list into the default JSON file name stored
    pub fn save(&self) {
        self.save_to_file(&self.file_name)
            .expect("Failed to save todo list to the default file");
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

        let deserialized_tasks: Vec<Task> = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to deserialize tasks: {}", e))?;

        self.tasks = deserialized_tasks;
        self.next_id = self
            .tasks
            .iter()
            .map(|task| {
                u32::from_str_radix(&task.id, 16)
                .expect(&format!("Failed to parse task ID: {}", task.id))
            })
            .max()
            .unwrap_or(0)
            + 1;
        Ok(())
    }
}
