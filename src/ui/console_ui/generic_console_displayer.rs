use crate::model::priority::Priority;
use crate::service::manager::{Manager, ManagerTrait};
use crate::ui::displayer::Displayer;
use crate::ui::line_editor::LineEditor;
use crate::ui::menu_option::{MENU_OPTIONS, MenuOption};
use std::io::{BufRead, Write};

/// Generic ConsoleDisplayer that implements all logic with customizable I/O.
pub struct GenericConsoleDisplayer<
    R: BufRead + Send + Sync,
    W: Write + Send + Sync,
    E: LineEditor + Send + Sync,
> {
    input: R,
    pub output: W,
    editor: E,
    buffer: String,
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync>
    GenericConsoleDisplayer<R, W, E>
{
    pub fn new(input: R, output: W, editor: E) -> Self {
        GenericConsoleDisplayer {
            input,
            output,
            editor,
            buffer: String::new(),
        }
    }

    /// Recieves two String messages to ask for the ID of the task and returns the id value
    fn _read_user_input(&mut self, messages: Vec<String>) -> Result<String, String> {
        let prompt = messages.join("\n");

        writeln!(self.output, "{}", prompt).map_err(|e| format!("Failed to write: {}", e))?;
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;

        self.buffer.clear();

        self.input
            .read_line(&mut self.buffer)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        Ok(self.buffer.trim().into())
    }

    pub fn handle_add_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: Add Task")
            .map_err(|e| format!("Failed to write: {}", e))?;

        let description = loop {
            let input = self._read_user_input(vec!["Enter task description:".into()])?;

            if input.is_empty() {
                writeln!(self.output, "Task description cannot be empty")
                    .map_err(|e| format!("Failed to write: {}", e))?;
            } else {
                break input;
            }
        };

        loop {
            let priority_input = self._read_user_input(vec![
                "Enter task priority number (1-High, 2-Medium, 3-Low):".into(),
            ])?;

            if let Ok(priority) = Priority::str_to_priority(&priority_input) {
                manager.add_task(description.as_ref(), &priority);
                writeln!(self.output, "Task added.")
                    .map_err(|e| format!("Failed to write: {}", e))?;
                break;
            } else {
                writeln!(
                    self.output,
                    "Invalid priority, please type again a valid one."
                )
                .map_err(|e| format!("Failed to write: {}", e))?;
            }
        }
        Ok(())
    }

    pub fn handle_list_tasks(&mut self, manager: &Manager) -> Result<(), String> {
        writeln!(self.output, "You selected: List Tasks")
            .map_err(|e| format!("Failed to write: {}", e))?;

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
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_toggle_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        let id_input = self._read_user_input(vec![
            "You selected: Complete Task".into(),
            "Enter task ID to complete:".into(),
        ])?;

        if manager.toggle_task_status(id_input.as_ref()) {
            writeln!(
                self.output,
                "Task with ID {} marked as completed.",
                id_input
            )
            .map_err(|e| format!("Failed to write: {}", e))?;
        } else {
            writeln!(self.output, "Task with ID {} not found.", id_input)
                .map_err(|e| format!("Failed to write: {}", e))?;
        }
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_remove_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        let id_input = self._read_user_input(vec![
            "You selected: Remove Task".into(),
            "Enter task ID to remove:".into(),
        ])?;

        let message = if manager.remove_task(id_input.as_ref()) {
            format!("Task with ID {} removed.", id_input)
        } else {
            format!("Task with ID {} not found.", id_input)
        };

        write!(self.output, "{}", message).map_err(|e| format!("Failed to write: {}", e))?;
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_edit_task(&mut self, manager: &mut Manager) -> Result<(), String> {
        let id_input = self._read_user_input(vec![
            "You selected: Edit Task".into(),
            "Enter task ID to edit".into(),
        ])?;

        if let Some(task) = manager.get_task(id_input.as_ref()) {
            let new_description = loop {
                let input = self
                    .editor
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
                let input = self
                    .editor
                    .readline_with_initial(
                        "Edit priority: ",
                        (task.priority.to_string().as_ref(), ""),
                    )
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
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    pub fn handle_redo(&mut self, manager: &mut Manager) -> Result<(), String> {
        match manager.redo() {
            Ok(true) => writeln!(self.output, "Redo operation successful.")
                .map_err(|e| format!("Failed to write: {}", e))?,
            Ok(false) => writeln!(self.output, "Redo operation failed, nothing to redo.")
                .map_err(|e| format!("Failed to write: {}", e))?,
            Err(e) => writeln!(self.output, "Redo failed: {}", e)
                .map_err(|e| format!("Failed to write: {}", e))?,
        }
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn handle_error(&mut self, error: &str) -> Result<(), String> {
        writeln!(self.output, "Error: {}", error).map_err(|e| format!("Failed to write: {}", e))?;
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }
}

impl<R: BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync> Displayer
    for GenericConsoleDisplayer<R, W, E>
{
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
                }
                Err(e) => {
                    let _ = self.handle_error(&e);
                }
            }
        }
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        writeln!(self.output, "\n\nToDo Operations:")
            .map_err(|e| format!("Failed to write: {}", e))?;

        for (text, _, _) in MENU_OPTIONS.iter() {
            writeln!(self.output, "{}", text).map_err(|e| format!("Failed to write: {}", e))?;
        }

        let input = self._read_user_input(vec!["Enter your choice (1-5): ".into()])?;

        MenuOption::str_to_menuoption(input.trim())
    }

    fn notify(&mut self, message: &str) -> Result<(), String> {
        writeln!(self.output, "[{}]", message).map_err(|e| format!("Failed to write: {}", e))?;
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }

    fn exit(&mut self) -> Result<(), String> {
        writeln!(self.output, "Exiting ToDo application... Goodbye!")
            .map_err(|e| format!("Failed to write: {}", e))?;
        self.output
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ui::{
        console_ui::generic_console_displayer::GenericConsoleDisplayer, line_editor::MockLineEditor,
    };
    use std::io::Cursor;

    #[test]
    fn test_handle_error() {
        let input = Cursor::new("".to_string());
        let output = Cursor::new(Vec::new());
        let editor = MockLineEditor::new(vec![]);
        let mut displayer = GenericConsoleDisplayer::new(input, output, editor);
        displayer
            .handle_error("Test error")
            .expect("Handle error failed");
        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Error: Test error"));
    }

    #[test]
    fn test_read_user_input_valid() {
        let input = Cursor::new("Test input\n".to_string());
        let output = Cursor::new(Vec::new());
        let editor = MockLineEditor::new(vec![]);
        let mut displayer = GenericConsoleDisplayer::new(input, output, editor);

        let result = displayer
            ._read_user_input(vec!["Enter something:".into()])
            .expect("Failed to read input");
        assert_eq!(result, "Test input");

        let output = String::from_utf8(displayer.output.into_inner()).unwrap();
        assert!(output.contains("Enter something:"));
    }
}
