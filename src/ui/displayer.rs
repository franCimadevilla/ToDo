use crate::service::manager::Manager;
use crate::service::menu_option::MenuOption;

/// Trait defining the interface for displaying and interacting with the ToDo application UI.
pub trait Displayer: Send + Sync {
    /// Creates a new instance of the displayer.
    fn new() -> Self
    where
        Self: Sized;

    fn run(&mut self, manager: &mut Manager);
    fn display(&mut self) -> Result<MenuOption, String>;
    fn notify(&mut self, message: &str) -> Result<(), String>;
    fn exit(&mut self) -> Result<(), String>;
}
