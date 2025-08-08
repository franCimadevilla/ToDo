use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::{Manager, ManagerTrait};
use crate::model::priority::Priority;
use std::io::{BufRead, BufReader, Write, Stdin, Stdout};

/// Generic ConsoleDisplayer that implements all logic with customizable I/O.
pub struct GenericConsoleDisplayer<R: BufRead + Send + Sync, W: Write + Send + Sync> {
    input: R,
    output: W,
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync> GenericConsoleDisplayer<R, W> {
    pub fn new(input: R, output: W) -> Self {
        GenericConsoleDisplayer { input, output }
    }

    fn handle_add_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: Add Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "Enter task description:").map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        let mut description = String::new();
        self.input
            .read_line(&mut description)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        let description = description.trim();

        writeln!(
            self.output,
            "Enter task priority number (1-High, 2-Medium, 3-Low):"
        )
        .map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        let mut priority_input = String::new();
        self.input
            .read_line(&mut priority_input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        let priority = match priority_input.trim() {
            "1" => Priority::High,
            "2" => Priority::Medium,
            "3" => Priority::Low,
            _ => {
                writeln!(self.output, "Invalid priority, defaulting to Low.")
                    .map_err(|e| format!("Failed to write: {}", e))?;
                Priority::Low
            }
        };
        manager.add_task(description.to_string(), priority);
        Ok(())
    }

    fn handle_list_tasks(&mut self, manager: &Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: List Tasks").map_err(|e| format!("Failed to write: {}", e))?;
        for task in manager.get_tasks() {
            writeln!(
                self.output,
                "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                task.id, task.description, task.priority, task.completed
            )
            .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_complete_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: Complete Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "Enter task ID to complete:").map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        let mut id_input = String::new();
        self.input
            .read_line(&mut id_input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        let id_input = id_input.trim();

        if manager.toggle_task_status(id_input.to_string()) {
            writeln!(self.output, "Task with ID {} marked as completed.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        } else {
            writeln!(self.output, "Task with ID {} not found.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_remove_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: Remove Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "Enter task ID to remove:").map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        let mut id_input = String::new();
        self.input
            .read_line(&mut id_input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        let id_input = id_input.trim();

        if manager.remove_task(id_input.to_string()) {
            writeln!(self.output, "Task with ID {} removed.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        } else {
            writeln!(self.output, "Task with ID {} not found.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_undo(&mut self, manager: &mut Manager) -> Result<(), String> {
        if let Err(e) = manager.undo() {
            writeln!(self.output, "Undo failed: {}", e).map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_redo(&mut self, manager: &mut Manager) -> Result<(), String> {
        match manager.redo() {
            Ok(true) => writeln!(self.output, "Redo operation successful.")
                .map_err(|e| format!("Failed to write: {}", e))?,
            Ok(false) => writeln!(self.output, "Redo operation failed, nothing to redo.")
                .map_err(|e| format!("Failed to write: {}", e))?,
            Err(e) => writeln!(self.output, "Redo failed: {}", e).map_err(|e| format!("Failed to write: {}", e))?,
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_error(&mut self, error: &str) -> Result<(), String> {
        writeln!(self.output, "Error: {}", error).map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync> Displayer for GenericConsoleDisplayer<R, W> {
    fn new() -> Self {
        panic!("Use GenericConsoleDisplayer::new(input, output) for testing");
    }

    fn run(&mut self, manager: &mut Manager) {
        let _ = self.notify("Welcome to the ToDo console application!");
        loop {
            match self.display() {
                Ok(MenuOption::AddTask) => {
                    let _ = self.handle_add_task(manager);
                }
                Ok(MenuOption::ListTasks) => {
                    let _ = self.handle_list_tasks(manager);
                }
                Ok(MenuOption::CompleteTask) => {
                    let _ = self.handle_complete_task(manager);
                }
                Ok(MenuOption::RemoveTask) => {
                    let _ = self.handle_remove_task(manager);
                }
                Ok(MenuOption::Exit) => {
                    let _ = self.exit();
                    break;
                }
                Ok(MenuOption::Undo) => {
                    let _ = self.handle_undo(manager);
                }
                Ok(MenuOption::Redo) => {
                    let _ = self.handle_redo(manager);
                }
                Err(e) => {
                    let _ = self.handle_error(&e);
                }
            }
        }
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        writeln!(self.output, "\nToDo Operations:").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "1. Add Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "2. List Tasks").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "3. Complete Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "4. Remove Task").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "5. Exit").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "[U] Undo").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "[R] Redo").map_err(|e| format!("Failed to write: {}", e))?;
        writeln!(self.output, "Enter your choice (1-5): ").map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;

        let mut input = String::new();
        self.input
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        match input.trim() {
            "1" => Ok(MenuOption::AddTask),
            "2" => Ok(MenuOption::ListTasks),
            "3" => Ok(MenuOption::CompleteTask),
            "4" => Ok(MenuOption::RemoveTask),
            "5" => Ok(MenuOption::Exit),
            "U" | "u" => Ok(MenuOption::Undo),
            "R" | "r" => Ok(MenuOption::Redo),
            _ => Err("Invalid option, please try again.".to_string()),
        }
    }

    fn notify(&mut self, message: &str) -> Result<(), String> {
        writeln!(self.output, "[{}]", message).map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn exit(&mut self) -> Result<(), String> {
        writeln!(self.output, "Exiting ToDo application... Goodbye!").map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

}

/// ConsoleDisplayer for production, wrapping GenericConsoleDisplayer with Stdin/Stdout.
pub struct ConsoleDisplayer {
    inner: GenericConsoleDisplayer<BufReader<Stdin>, Stdout>,
}

impl ConsoleDisplayer {
    pub fn new() -> Self {
        ConsoleDisplayer {
            inner: GenericConsoleDisplayer::new(
                BufReader::new(std::io::stdin()),
                std::io::stdout(),
            ),
        }
    }
}

impl Displayer for ConsoleDisplayer {
    fn new() -> Self {
        ConsoleDisplayer::new()
    }

    fn run(&mut self, manager: &mut Manager) {
        self.inner.run(manager)
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        self.inner.display()
    }

    fn notify(&mut self, message: &str) -> Result<(), String> {
        return self.inner.notify(message);
    
    }

    fn exit(&mut self) -> Result<(), String> {
        return self.inner.exit();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_manager_with_tasks() -> Manager {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer);
        let mut manager = Manager::new(displayer);
        manager.add_task("Test Task 1".to_string(), Priority::High);
        manager.add_task("Test Task 2".to_string(), Priority::Medium);
        manager
    }

    struct MockDisplayer;
    impl Displayer for MockDisplayer {
        fn new() -> Self {
            MockDisplayer
        }
        fn run(&mut self, _manager: &mut Manager) {}
        fn display(&mut self) -> Result<MenuOption, String> {
            Ok(MenuOption::Exit)
        }
        fn notify(&mut self, _message: &str) -> Result<(), String> { Ok(())}
        fn exit(&mut self) -> Result<(), String> { Ok(())}
    }

    #[test]
    fn test_notify() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        displayer.notify("Test message").expect("Notify failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert_eq!(output, "[Test message]\n");
    }

    #[test]
    fn test_exit() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        displayer.exit().expect("Exit failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert_eq!(output, "Exiting ToDo application... Goodbye!\n");
    }

    #[test]
    fn test_display_add_task() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let result = displayer.display().expect("Display failed");
        assert_eq!(result, MenuOption::AddTask);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("ToDo Operations:"));
        assert!(output.contains("1. Add Task"));
        assert!(output.contains("Enter your choice (1-5):"));
    }

    #[test]
    fn test_display_invalid_option() {
        let input = Cursor::new("invalid\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let result = displayer.display();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid option, please try again.");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("ToDo Operations:"));
    }

    #[test]
    fn test_handle_add_task() {
        let input = Cursor::new("Test Task\n2\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_add_task(&mut manager).expect("Add task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("You selected: Add Task"));
        assert!(output.contains("Enter task description:"));
        assert!(output.contains("Enter task priority number"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.description == "Test Task" && t.priority == Priority::Medium));
    }

    #[test]
    fn test_handle_add_task_invalid_priority() {
        let input = Cursor::new("Test Task\ninvalid\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_add_task(&mut manager).expect("Add task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Invalid priority, defaulting to Low"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.description == "Test Task" && t.priority == Priority::Low));
    }

    #[test]
    fn test_handle_list_tasks() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let manager = create_manager_with_tasks();
        displayer.handle_list_tasks(&manager).expect("List tasks failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("You selected: List Tasks"));
        assert!(output.contains("Test Task 1"));
        assert!(output.contains("Test Task 2"));
    }

    #[test]
    fn test_handle_complete_task_success() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_complete_task(&mut manager).expect("Complete task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("You selected: Complete Task"));
        assert!(output.contains("Task with ID 1 marked as completed"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().find(|t| t.id == "1").unwrap().completed);
    }

    #[test]
    fn test_handle_complete_task_not_found() {
        let input = Cursor::new("999\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_complete_task(&mut manager).expect("Complete task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Task with ID 999 not found"));
    }

    #[test]
    fn test_handle_remove_task_success() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_remove_task(&mut manager).expect("Remove task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("You selected: Remove Task"));
        assert!(output.contains("Task with ID 1 removed"));
        let tasks = manager.get_tasks();
        assert!(!tasks.iter().any(|t| t.id == "1"));
    }

    #[test]
    fn test_handle_remove_task_not_found() {
        let input = Cursor::new("999\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.handle_remove_task(&mut manager).expect("Remove task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Task with ID 999 not found"));
    }

    #[test]
    fn test_handle_undo() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        manager.remove_task("1".to_string());
        displayer.handle_undo(&mut manager).expect("Undo failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.id == "1"));
        assert!(!output.contains("Undo failed"));
    }

    #[test]
    fn test_handle_redo() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        let id_to_remove = manager.get_tasks()[0].id.clone();
        manager.remove_task(id_to_remove);
        manager.undo().expect("Undo failed");
        displayer.handle_redo(&mut manager).expect("Redo failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Redo operation successful"));
        let tasks = manager.get_tasks();
        assert!(!tasks.iter().any(|t| t.id == "1"));
    }

    #[test]
    fn test_handle_error() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        displayer.handle_error("Test error").expect("Handle error failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Error: Test error"));
    }

    #[test]
    fn test_run_add_and_exit() {
        let input = Cursor::new("1\nTest Task\n2\n5\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Welcome to the ToDo console application!"));
        assert!(output.contains("You selected: Add Task"));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.description == "Test Task" && t.priority == Priority::Medium));
    }

    #[test]
    fn test_run_invalid_input() {
        let input = Cursor::new("invalid\n5\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Error: Invalid option, please try again."));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
    }

    #[test]
    fn test_console_displayer_run() {
        let input = Cursor::new("1\nTest Task\n2\n5\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Welcome to the ToDo console application!"));
        assert!(output.contains("You selected: Add Task"));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.description == "Test Task" && t.priority == Priority::Medium));
    }
}
