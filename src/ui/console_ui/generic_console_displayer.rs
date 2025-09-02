use crate::ui::displayer_trait::Displayer;
use crate::service::menu_option::{MenuOption, MENU_OPTIONS};
use crate::service::manager::{Manager, ManagerTrait};
use crate::service::line_editor::LineEditor;
use crate::model::priority::Priority;
use std::io::{BufRead, Write};


/// Generic ConsoleDisplayer that implements all logic with customizable I/O.
pub struct GenericConsoleDisplayer<R: BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync> {
    input: R,
    output: W,
    editor: E
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync> GenericConsoleDisplayer<R, W, E> {
    pub fn new(input: R, output: W, editor : E) -> Self {
        GenericConsoleDisplayer { input, output, editor}
    }

    pub fn handle_add_task(&mut self, manager: &mut Manager) -> Result<(), String> {

        writeln!(self.output, "You selected: Add Task")
            .map_err(|e| format!("Failed to write: {}", e))?;

        
        let description = loop {
            let input = self._helper_ask_str_input(vec![
                "Enter task description:".into()
            ])?;

            if input.is_empty() {
                writeln!(self.output, "Task description cannot be empty")
                    .map_err(|e| format!("Failed to write: {}", e))?;
            } else {
                break input;
            }
        };

        
        loop {
            let priority_input = self._helper_ask_str_input(vec![
                "Enter task priority number (1-High, 2-Medium, 3-Low):".into()
            ])?;
            
            if let Ok(priority) = Priority::str_to_priority(&priority_input) {
                manager.add_task(description.as_ref(), &priority);
                writeln!(self.output, "Task added.")
                    .map_err(|e| format!("Failed to write: {}", e))?;
                break;
            } else {
                writeln!(self.output, "Invalid priority, please type again a valid one.")
                    .map_err(|e| format!("Failed to write: {}", e))?;
            } 
        }
        Ok(())
    }

    pub fn handle_list_tasks(&mut self, manager: &Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: List Tasks").map_err(|e| format!("Failed to write: {}", e))?;

        if manager.get_tasks().is_empty() { 
            writeln!(self.output, "No tasks in the list.")
                .map_err(|e| format!("Failed to write: {}", e))?; 
        } else {
            for task in manager.get_tasks() {
                writeln!(
                    self.output,
                    "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                    task.id, task.description, task.priority, task.completed
                )
                .map_err(|e| format!("Failed to write: {}", e))?;
            }
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_toggle_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        
        let id_input = self._helper_ask_str_input(vec![
            "You selected: Complete Task".into(),
            "Enter task ID to complete:".into()
        ])?;

        if manager.toggle_task_status(id_input.as_ref()) {
            writeln!(self.output, "Task with ID {} marked as completed.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        } else {
            writeln!(self.output, "Task with ID {} not found.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_remove_task(&mut self, manager: &mut Manager) -> Result<(), String> {

        let id_input = self._helper_ask_str_input(vec![
                "You selected: Remove Task".into(), 
                "Enter task ID to remove:".into()
            ])?;

        let message = if manager.remove_task(id_input.as_ref()) {
            format!("Task with ID {} removed.", id_input)
        } else {
            format!("Task with ID {} not found.", id_input)
        };

        write!(self.output, "{}", message).map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }
        
    pub fn handle_edit_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        let id_input = self._helper_ask_str_input(vec![
            "You selected: Edit Task".into(),
            "Enter task ID to edit".into()
        ])?;

        if let Some(task) = manager.get_task(id_input.as_ref()) {
            let new_description = loop { 
                let input =  self.editor
                        .readline_with_initial("Edit description: ", (task.description.as_ref(), ""))
                        .map_err(|e| format!("Failed in the line editor: {}", e))?;

                if input.is_empty() {
                    writeln!(self.output, "Task description cannot be empty")
                    .map_err(|e| format!("Failed to write: {}", e))?;
                } else {
                    break input;
                }
            };
            
            let new_priority = loop {
                let input =  self.editor
                        .readline_with_initial("Edit priority: ", (task.priority.to_string().as_ref(), ""))
                        .map_err(|e| format!("Failed in the line editor: {}", e))?;

                if let Ok(priority) = Priority::str_to_priority(input.as_ref()) {
                    break priority;
                }
            };

            if manager.edit_task(&id_input, &new_description, &new_priority) {
                write!(self.output, "Task with ID: {} was edited", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
            }

        } else {
            write!(self.output, "Task with ID: {} not found", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        Ok(()) 
    }

    pub fn handle_undo(&mut self, manager: &mut Manager) -> Result<(), String> {
        if let Err(e) = manager.undo() {
            writeln!(self.output, "Undo failed: {}", e)
                .map_err(|e| format!("Failed to write: {}", e))?;
        } else {
            writeln!(self.output, "Undo operation successful.")
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_redo(&mut self, manager: &mut Manager) -> Result<(), String> {
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

    /// Recieves two String messages to ask for the ID of the task and returns the id value
    fn _helper_ask_str_input(&mut self, messages : Vec<String>) -> Result<String, String> {
        
        let prompt = messages.join("\n");

        writeln!(self.output, "{}", prompt).map_err(|e| format!("Failed to write: {}", e))?;
        self.output.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        let mut str_input = String::new();
        self.input
            .read_line(&mut str_input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        Ok(str_input.trim().into())
    }
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync> Displayer for GenericConsoleDisplayer<R, W, E> {
    fn new() -> Self {
        panic!("Use GenericConsoleDisplayer::new(input, output) for testing");
    }

    fn run(&mut self, manager: &mut Manager) {
        let _ = self.notify("Welcome to the ToDo console application!");
        loop {
            match self.display() {
                Ok(option) => {
                    if let Ok(continue_) = option.execute(self, manager) {
                        if !continue_ { 
                            break;
                        }
                    }
                },
                Err(e) => {
                    let _ = self.handle_error(&e);
                }

            }
        }
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        writeln!(self.output, "\n\nToDo Operations:").map_err(|e| format!("Failed to write: {}", e))?;
       
       for (text, _, _) in MENU_OPTIONS.iter() {
            writeln!(self.output, "{}", text).map_err(|e| format!("Failed to write: {}", e))?;
       }

       let input = self._helper_ask_str_input(vec![
        "Enter your choice (1-5): ".into()
       ])?;

        MenuOption::str_to_menuoption(input.trim())
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use crate::service::line_editor::MockLineEditor;
    use crate::ui::displayer_trait::MockDisplayer;

    fn create_manager_with_tasks() -> Manager {
        let displayer: Box<dyn Displayer> = Box::new(MockDisplayer);
        let mut manager = Manager::new(displayer);
        manager.add_task("Test Task 1".as_ref(), &Priority::High);
        manager.add_task("Test Task 2".as_ref(), &Priority::Medium);
        manager
    }

    fn create_displayer_mocked_editor ( input : Cursor<String>, output : Cursor<Vec<u8>>, editor_vec : Vec<String>) 
        -> GenericConsoleDisplayer<Cursor<String>, Cursor<Vec<u8>>, MockLineEditor> 
    {
        let editor = MockLineEditor::new(editor_vec);
        GenericConsoleDisplayer::new(input, output, editor)
    }

    #[test]
    fn test_notify() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        displayer.notify("Test message").expect("Notify failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert_eq!(output, "[Test message]\n");
    }

    #[test]
    fn test_exit() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        displayer.exit().expect("Exit failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert_eq!(output, "Exiting ToDo application... Goodbye!\n");
    }

    #[test]
    fn test_display_add_task() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
        let input = Cursor::new("Test Task\ninvalid\n2\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.handle_add_task(&mut manager).expect("Add task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Invalid priority, please type again a valid one."));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().any(|t| t.description == "Test Task" && t.priority == Priority::Medium));
    }

    #[test]
    fn test_handle_list_tasks() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let manager = create_manager_with_tasks();
        displayer.handle_list_tasks(&manager).expect("List tasks failed");
        let output = String::from_utf8(displayer.output.into_inner()).expect("Failed to convert output to string");
        assert!(output.contains("You selected: List Tasks"));
        assert!(output.contains("Test Task 1"));
        assert!(output.contains("Test Task 2"));
    }

    #[test]
    fn test_handle_toggle_task_success() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.handle_toggle_task(&mut manager).expect("Complete task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("You selected: Complete Task"));
        assert!(output.contains("Task with ID 1 marked as completed"));
        let tasks = manager.get_tasks();
        assert!(tasks.iter().find(|t| t.id == "1").unwrap().completed);
    }

    #[test]
    fn test_handle_toggle_task_not_found() {
        let input = Cursor::new("999\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.handle_toggle_task(&mut manager).expect("Complete task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Task with ID 999 not found"));
    }

    #[test]
    fn test_handle_remove_task_success() {
        let input = Cursor::new("1\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.handle_remove_task(&mut manager).expect("Remove task failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Task with ID 999 not found"));
    }

    #[test]
    fn test_handle_undo() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        manager.remove_task("1".as_ref());
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
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        let id_to_remove = manager.get_tasks()[0].id.clone();
        manager.remove_task(id_to_remove.as_ref());
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
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        displayer.handle_error("Test error").expect("Handle error failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Error: Test error"));
    }

    #[test]
    fn test_run_add_and_exit() {
        let input = Cursor::new("1\nTest Task\n2\ne\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
        let input = Cursor::new("invalid\ne\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Error: Invalid option, please try again."));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
    }

    
    #[test]
    fn test_console_displayer_run() {
        let input = Cursor::new("1\nTest Task\n2\ne\n".to_string());
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
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
    fn test_handle_edit_task_no_change() {
        let input_vec = vec![   // List, Edit task, new description, new priority, List, Exit
            MenuOption::get_input_key(&MenuOption::AddTask).to_string(),
            "Test Description".into(),
            "3".into(),
            MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
            MenuOption::get_input_key(&MenuOption::EditTask).to_string(),
             "Test Description".into(),
            "3".into(),
            MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
            MenuOption::get_input_key(&MenuOption::Exit).to_string(),
        ];
        let input = input_vec.join("\n");
        let input = Cursor::new(input);
        let output = Cursor::new(Vec::new());
        let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Welcome to the ToDo console application!"));
        assert!(output.contains("You selected: Add Task"));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
    }

    #[test]
    fn test_handle_edit_task_change_fields() {
        let input_vec = vec![   // List, Edit task, id, new description, new priority, List, Exit
            MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
            MenuOption::get_input_key(&MenuOption::EditTask).to_string(),
            "1".into(), //ID
            "New Description".into(),
            "1".into(),
            MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
            MenuOption::get_input_key(&MenuOption::Exit).to_string(),
        ];
        let mut input = input_vec.join("\n");
        input.push_str("\n");

        assert_eq!(input, "2\n5\n1\nNew Description\n1\n2\ne\n");
      
        let input = Cursor::new(input);
        let output = Cursor::new(Vec::new());
        let editor = MockLineEditor::new(vec!["New Description".into(), "1".into()]);
        let mut displayer = GenericConsoleDisplayer::new(input, output, editor);
        let mut manager = create_manager_with_tasks();
        displayer.run(&mut manager);
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();

        assert!(output.contains("Welcome to the ToDo console application!"));
        assert!(output.contains("Description: Test Task 1, Priority: High, Completed: false"));
        assert!(output.contains("You selected: Edit Task"));
        assert!(output.contains("Description: New Description, Priority: High, Completed: false"));
        assert!(output.contains("Exiting ToDo application... Goodbye!"));
    }

}
