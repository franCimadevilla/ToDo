use once_cell::sync::Lazy;
use crate::{service::{line_editor::LineEditor, manager::Manager}, ui::{console_ui::generic_console_displayer::GenericConsoleDisplayer, displayer_trait::Displayer}};
use std::io::{BufRead, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuOption {
    AddTask,
    ListTasks,
    CompleteTask,
    RemoveTask,
    EditTask,
    Exit,
    Undo,
    Redo,
}

impl MenuOption {
    pub fn str_to_menuoption(text : &str) -> Result<MenuOption, String> {
        let input = text.to_lowercase();
        MENU_OPTIONS
            .iter()
            .find(|(_, key, _)| key.eq_ignore_ascii_case(&input))
            .map(|(_, _, option)| *option)
            .ok_or_else(|| format!("The option: {} is invalid, please try again.", text))
    }

    pub fn get_input_key(menuoption : &MenuOption) -> &str {
        MENU_OPTIONS
            .iter()
            .find(|(_, _, option )| option.eq(menuoption))
            .map(|(_, key, _)| key)
            .expect("Invalid option")
    }

    pub fn execute<R:BufRead + Send + Sync, W: Write + Send + Sync, E: LineEditor + Send + Sync>(
        &self,
        displayer : &mut GenericConsoleDisplayer<R, W, E>,
        manager : &mut Manager
    ) -> Result<bool, String> {
         match self {
            MenuOption::AddTask => displayer.handle_add_task(manager),
            MenuOption::ListTasks => displayer.handle_list_tasks(manager),
            MenuOption::CompleteTask => displayer.handle_toggle_task(manager),
            MenuOption::RemoveTask => displayer.handle_remove_task(manager),
            MenuOption::EditTask => displayer.handle_edit_task(manager),
            MenuOption::Exit => {
                let _ = displayer.exit();
                return Ok(false); // señal para salir del bucle
            }
            MenuOption::Undo => displayer.handle_undo(manager),
            MenuOption::Redo => displayer.handle_redo(manager),
        }?;
        Ok(true)
    }
}

pub static MENU_OPTIONS: Lazy<Vec<(&'static str, &'static str, MenuOption)>> = Lazy::new(|| {
    vec![
        ("1. Add Task", "1", MenuOption::AddTask),
        ("2. List Tasks", "2", MenuOption::ListTasks),
        ("3. Complete Task", "3", MenuOption::CompleteTask),
        ("4. Remove Task", "4", MenuOption::RemoveTask),
        ("5. Edit Task", "5", MenuOption::EditTask),
        ("[E] Exit", "e", MenuOption::Exit),
        ("[U] Undo", "u", MenuOption::Undo),
        ("[R] Redo", "r", MenuOption::Redo),
    ]
});