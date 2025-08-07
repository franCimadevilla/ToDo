
use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::{Manager, ManagerTrait};
use crate::model::priority::Priority;

/// ConsoleDisplayer implements the Displayer trait for console output.
pub struct ConsoleDisplayer;

impl Displayer for ConsoleDisplayer {
    fn new() -> Self {
        ConsoleDisplayer
    }
    fn run(&mut self, manager : &mut Manager) {
        println!("Welcome to the ToDo console application!");

        loop {
            match self.display() {
                Ok(MenuOption::AddTask) => {
                    println!("You selected: Add Task");
                    println!("Enter task description:");
                    let mut description = String::new();
                    std::io::stdin().read_line(&mut description)
                        .expect("Failed to read line");
                    println!("Enter task priority number (1-High, 2-Medium, 3-Low):");
                    let mut priority_input = String::new();
                    std::io::stdin().read_line(&mut priority_input)
                        .expect("Failed to read line");
                    let priority = match priority_input.trim() {
                        "1" => Priority::High,
                        "2" => Priority::Medium,
                        "3" => Priority::Low,
                        _ => {
                            println!("Invalid priority, defaulting to Low.");
                            Priority::Low
                        }
                    };
                    manager.add_task(description.trim().to_string(), priority);
                    
                },
                Ok(MenuOption::ListTasks) => {
                    println!("You selected: List Tasks");
                    for task in manager.get_tasks() {
                    println!("ID: {}, Description: {}, Priority: {:?}, Completed: {}", 
                     task.id, task.description, task.priority, task.completed);
                }
                },
                Ok(MenuOption::CompleteTask) => {
                    println!("You selected: Complete Task");
                    println!("Enter task ID to complete:");
                    let mut id_input = String::new();
                    std::io::stdin().read_line(&mut id_input)
                        .expect("Failed to read line");
                    
                    if manager.complete_task(id_input.clone()) {
                        println!("Task with ID {} marked as completed.", id_input);
                    } else {
                        println!("Task with ID {} not found.", id_input);
                    }
                    
                },
                Ok(MenuOption::RemoveTask) => {
                    println!("You selected: Remove Task");
                    println!("Enter task ID to remove:");
                    let mut id_input = String::new();
                    std::io::stdin().read_line(&mut id_input)
                        .expect("Failed to read line");
                    
                    if manager.remove_task(id_input.clone()) {
                        println!("Task with ID {} removed.", id_input);
                    } else {
                        println!("Task with ID {} not found.", id_input);
                    }
                    
                },
                Ok(MenuOption::Exit) => {
                    self.exit();
                    break;
                },
                Ok(MenuOption::Undo) => {
                    if let Err(e) = manager.undo() {
                        println!("Undo failed: {}", e);
                    }
                },
                Ok(MenuOption::Redo) => {
                    match manager.redo() {
                        Ok(true) => println!("Redo operation successful."),
                        Ok(false) => println!("Redo operation failed, nothing to redo."),
                        _ => println!("Redo failed"),
                    }
                },
                Err(e) => println!("Error: {}", e),
            }

        }
    }

    fn display(&self) -> Result<MenuOption, String> {
        println!("\nToDo Operations:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Remove Task");
        println!("5. Exit");
        println!("[U] Undo");
        println!("[R] Redo");
        println!("Enter your choice (1-5): ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        match input.trim() {
            "1" => Ok(MenuOption::AddTask),
            "2" => Ok(MenuOption::ListTasks),
            "3" => Ok(MenuOption::CompleteTask),
            "4" => Ok(MenuOption::RemoveTask),
            "5" => Ok(MenuOption::Exit),
            "U"|"u" => Ok(MenuOption::Undo),
            "R"|"r" => Ok(MenuOption::Redo),
            _ => Err("Invalid option, please try again.".to_string()),
        }
    }

    fn notify(&self, message: &str) {
        println!("[{}]", message);
    }
    fn exit(&self) {
        println!("Exiting ToDo application... Goodbye!");
    }
}