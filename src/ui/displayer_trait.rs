use crate::service::menu_option::MenuOption;
use crate::service::manager::Manager;

pub trait Displayer: Send + Sync {
    fn new() -> Self where Self: Sized;

    fn run(&mut self, manager: &mut Manager);
    fn display(&mut self) -> Result<MenuOption, String>;  
    fn notify(&mut self, message: &str) -> Result<(), String>;  
    fn exit(&mut self) -> Result<(), String>;  
}

pub struct MockDisplayer;

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