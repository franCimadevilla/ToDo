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
                "IllegalArgument Error: Task with ID {} not found when trying to toggle its state.",
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
                "IllegalArgument Error: Task with ID {} not found when trying to remove.",
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
        todo_list.toggle_task_status(id.clone());
        assert_eq!(todo_list.tasks[0].completed, true);
        // Toggle again to test flipping back
        todo_list.toggle_task_status(id.clone());
        assert_eq!(todo_list.tasks[0].completed, false);
    }

    #[test]
    #[should_panic(expected = "IllegalArgument Error: Task with ID notfound not found")]
    fn test_complete_task_not_found() {
        let mut todo_list = TodoList::new();
        todo_list.toggle_task_status("notfound".to_string());
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".to_string(), Priority::Low);
        todo_list.remove_task(id.clone());
        assert_eq!(todo_list.tasks.len(), 0);
    }

    #[test]
    #[should_panic(expected = "IllegalArgument Error: Task with ID notfound not found")]
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

    #[test]
    fn test_try_load_no_file() {
        let mut todo_list = TodoList::new();
        let result = todo_list.load_from_file("no_exist.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_load_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("todo.json");
        std::fs::write(&file_path, "invalid json").unwrap();
        let mut todo_list = TodoList::new();
        let result = todo_list.load_from_file("todo.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_task() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".into(), Priority::High);
        let new_text = "Edited text".into();
        let new_priority = Priority::Low;
        todo_list.edit_task(id.as_ref(), (new_text, &new_priority));

        let edited_task = todo_list.get_task_mut(id.as_ref()).expect("Task not found");
        assert_eq!(edited_task.description, new_text.to_string())
    }

    #[test]
    fn test_get_task_mut_exist() {
        let mut todo_list = TodoList::new();
        let id = todo_list.add_task("Test task".into(), Priority::High);

        let task = todo_list.get_task_mut(id.as_ref());
        assert!(task.is_some())
    }

    #[test]
    fn test_task_mut_not_found() {
        let mut todo_list = TodoList::new();
        let id = "2".to_string();
        let task = todo_list.get_task_mut(id.as_ref());
        assert!(task.is_none())
    }
}
