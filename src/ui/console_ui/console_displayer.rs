use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::{Manager, ManagerTrait};
use crate::model::priority::Priority;
use std::io::{BufRead, Write}; // Changed Read to BufRead

/// Concrete ConsoleDisplayer that use stdin/stdout.
pub struct ConsoleDisplayer {
    input: std::io::Stdin,
    output: std::io::Stdout,
}

/// Generic ConsoleDisplayer for testing with custom BufRead/Write types.
pub struct GenericConsoleDisplayer<R: BufRead + Send + Sync, W: Write + Send + Sync> {
    input: R,
    output: W,
}

impl ConsoleDisplayer {
    pub fn new() -> Self {
        ConsoleDisplayer {
            input: std::io::stdin(),
            output: std::io::stdout(),
        }
    }
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync> GenericConsoleDisplayer<R, W> {
    pub fn new(input: R, output: W) -> Self {
        GenericConsoleDisplayer { input, output }
    }
}

impl Displayer for ConsoleDisplayer {
    fn new() -> Self {
        ConsoleDisplayer::new()
    }

    fn run(&mut self, manager: &mut Manager) {
        writeln!(self.output, "Welcome to the ToDo console application!").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");

        loop {
            match self.display() {
                Ok(MenuOption::AddTask) => {
                    writeln!(self.output, "You selected: Add Task").expect("Failed to write");
                    writeln!(self.output, "Enter task description:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut description = String::new();
                    self.input
                        .read_line(&mut description)
                        .expect("Failed to read input");
                    let description = description.trim();

                    writeln!(
                        self.output,
                        "Enter task priority number (1-High, 2-Medium, 3-Low):"
                    )
                    .expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut priority_input = String::new();
                    self.input
                        .read_line(&mut priority_input)
                        .expect("Failed to read input");
                    let priority = match priority_input.trim() {
                        "1" => Priority::High,
                        "2" => Priority::Medium,
                        "3" => Priority::Low,
                        _ => {
                            writeln!(self.output, "Invalid priority, defaulting to Low.")
                                .expect("Failed to write");
                            Priority::Low
                        }
                    };
                    manager.add_task(description.to_string(), priority);
                }
                Ok(MenuOption::ListTasks) => {
                    writeln!(self.output, "You selected: List Tasks").expect("Failed to write");
                    for task in manager.get_tasks() {
                        writeln!(
                            self.output,
                            "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                            task.id, task.description, task.priority, task.completed
                        )
                        .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::CompleteTask) => {
                    writeln!(self.output, "You selected: Complete Task").expect("Failed to write");
                    writeln!(self.output, "Enter task ID to complete:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut id_input = String::new();
                    self.input
                        .read_line(&mut id_input)
                        .expect("Failed to read input");
                    let id_input = id_input.trim();

                    if manager.complete_task(id_input.to_string()) {
                        writeln!(self.output, "Task with ID {} marked as completed.", id_input)
                            .expect("Failed to write");
                    } else {
                        writeln!(self.output, "Task with ID {} not found.", id_input)
                            .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::RemoveTask) => {
                    writeln!(self.output, "You selected: Remove Task").expect("Failed to write");
                    writeln!(self.output, "Enter task ID to remove:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut id_input = String::new();
                    self.input
                        .read_line(&mut id_input)
                        .expect("Failed to read input");
                    let id_input = id_input.trim();

                    if manager.remove_task(id_input.to_string()) {
                        writeln!(self.output, "Task with ID {} removed.", id_input)
                            .expect("Failed to write");
                    } else {
                        writeln!(self.output, "Task with ID {} not found.", id_input)
                            .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::Exit) => {
                    self.exit();
                    break;
                }
                Ok(MenuOption::Undo) => {
                    if let Err(e) = manager.undo() {
                        writeln!(self.output, "Undo failed: {}", e).expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::Redo) => {
                    match manager.redo() {
                        Ok(true) => writeln!(self.output, "Redo operation successful.")
                            .expect("Failed to write"),
                        Ok(false) => writeln!(self.output, "Redo operation failed, nothing to redo.")
                            .expect("Failed to write"),
                        Err(e) => writeln!(self.output, "Redo failed: {}", e).expect("Failed to write"),
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Err(e) => {
                    writeln!(self.output, "Error: {}", e).expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                }
            }
        }
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        writeln!(self.output, "\nToDo Operations:").expect("Failed to write");
        writeln!(self.output, "1. Add Task").expect("Failed to write");
        writeln!(self.output, "2. List Tasks").expect("Failed to write");
        writeln!(self.output, "3. Complete Task").expect("Failed to write");
        writeln!(self.output, "4. Remove Task").expect("Failed to write");
        writeln!(self.output, "5. Exit").expect("Failed to write");
        writeln!(self.output, "[U] Undo").expect("Failed to write");
        writeln!(self.output, "[R] Redo").expect("Failed to write");
        writeln!(self.output, "Enter your choice (1-5): ").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");

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

    fn notify(&mut self, message: &str) {
        writeln!(self.output, "[{}]", message).expect("Failed to write");
        self.output.flush().expect("Failed to flush output");
    }

    fn exit(&mut self) {
        writeln!(self.output, "Exiting ToDo application... Goodbye!").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");
    }
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync> Displayer for GenericConsoleDisplayer<R, W> {
    fn new() -> Self {
        panic!("Error: Use GenericConsoleDisplayer::new(input, output) only for testing");
    }

    fn run(&mut self, manager: &mut Manager) {
        writeln!(self.output, "Welcome to the ToDo console application!").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");

        loop {
            match self.display() {
                Ok(MenuOption::AddTask) => {
                    writeln!(self.output, "You selected: Add Task").expect("Failed to write");
                    writeln!(self.output, "Enter task description:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut description = String::new();
                    self.input
                        .read_line(&mut description)
                        .expect("Failed to read input");
                    let description = description.trim();

                    writeln!(
                        self.output,
                        "Enter task priority number (1-High, 2-Medium, 3-Low):"
                    )
                    .expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut priority_input = String::new();
                    self.input
                        .read_line(&mut priority_input)
                        .expect("Failed to read input");
                    let priority = match priority_input.trim() {
                        "1" => Priority::High,
                        "2" => Priority::Medium,
                        "3" => Priority::Low,
                        _ => {
                            writeln!(self.output, "Invalid priority, defaulting to Low.")
                                .expect("Failed to write");
                            Priority::Low
                        }
                    };
                    manager.add_task(description.to_string(), priority);
                }
                Ok(MenuOption::ListTasks) => {
                    writeln!(self.output, "You selected: List Tasks").expect("Failed to write");
                    for task in manager.get_tasks() {
                        writeln!(
                            self.output,
                            "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                            task.id, task.description, task.priority, task.completed
                        )
                        .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::CompleteTask) => {
                    writeln!(self.output, "You selected: Complete Task").expect("Failed to write");
                    writeln!(self.output, "Enter task ID to complete:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut id_input = String::new();
                    self.input
                        .read_line(&mut id_input)
                        .expect("Failed to read input");
                    let id_input = id_input.trim();

                    if manager.complete_task(id_input.to_string()) {
                        writeln!(self.output, "Task with ID {} marked as completed.", id_input)
                            .expect("Failed to write");
                    } else {
                        writeln!(self.output, "Task with ID {} not found.", id_input)
                            .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::RemoveTask) => {
                    writeln!(self.output, "You selected: Remove Task").expect("Failed to write");
                    writeln!(self.output, "Enter task ID to remove:").expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                    let mut id_input = String::new();
                    self.input
                        .read_line(&mut id_input)
                        .expect("Failed to read input");
                    let id_input = id_input.trim();

                    if manager.remove_task(id_input.to_string()) {
                        writeln!(self.output, "Task with ID {} removed.", id_input)
                            .expect("Failed to write");
                    } else {
                        writeln!(self.output, "Task with ID {} not found.", id_input)
                            .expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::Exit) => {
                    self.exit();
                    break;
                }
                Ok(MenuOption::Undo) => {
                    if let Err(e) = manager.undo() {
                        writeln!(self.output, "Undo failed: {}", e).expect("Failed to write");
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Ok(MenuOption::Redo) => {
                    match manager.redo() {
                        Ok(true) => writeln!(self.output, "Redo operation successful.")
                            .expect("Failed to write"),
                        Ok(false) => writeln!(self.output, "Redo operation failed, nothing to redo.")
                            .expect("Failed to write"),
                        Err(e) => writeln!(self.output, "Redo failed: {}", e).expect("Failed to write"),
                    }
                    self.output.flush().expect("Failed to flush output");
                }
                Err(e) => {
                    writeln!(self.output, "Error: {}", e).expect("Failed to write");
                    self.output.flush().expect("Failed to flush output");
                }
            }
        }
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        writeln!(self.output, "\nToDo Operations:").expect("Failed to write");
        writeln!(self.output, "1. Add Task").expect("Failed to write");
        writeln!(self.output, "2. List Tasks").expect("Failed to write");
        writeln!(self.output, "3. Complete Task").expect("Failed to write");
        writeln!(self.output, "4. Remove Task").expect("Failed to write");
        writeln!(self.output, "5. Exit").expect("Failed to write");
        writeln!(self.output, "[U] Undo").expect("Failed to write");
        writeln!(self.output, "[R] Redo").expect("Failed to write");
        writeln!(self.output, "Enter your choice (1-5): ").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");

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

    fn notify(&mut self, message: &str) {
        writeln!(self.output, "[{}]", message).expect("Failed to write");
        self.output.flush().expect("Failed to flush output");
    }

    fn exit(&mut self) {
        writeln!(self.output, "Exiting ToDo application... Goodbye!").expect("Failed to write");
        self.output.flush().expect("Failed to flush output");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_notify() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        displayer.notify("Test message");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert_eq!(output, "[Test message]\n");
    }

    #[test]
    fn test_exit() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = GenericConsoleDisplayer::new(input, output);
        displayer.exit();
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
}