use crate::ui::console_ui::menu_option::MenuOption;
use crate::service::manager::Manager;

pub trait Displayer: Send + Sync {
    fn new() -> Self where Self: Sized;

    fn run(&mut self, manager: &mut Manager);
    fn display(&mut self) -> Result<MenuOption, String>;  
    fn notify(&mut self, message: &str) -> Result<(), String>;  
    fn exit(&mut self) -> Result<(), String>;  
}