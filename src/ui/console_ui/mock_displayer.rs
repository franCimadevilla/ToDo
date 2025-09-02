use crate::service::manager::Manager;
use crate::service::menu_option::MenuOption;
use crate::ui::displayer::Displayer;

pub struct MockDisplayer;

impl Displayer for MockDisplayer {
    fn new() -> Self {
        MockDisplayer
    }
    fn run(&mut self, _manager: &mut Manager) {}
    fn display(&mut self) -> Result<MenuOption, String> {
        Ok(MenuOption::Exit)
    }
    fn notify(&mut self, _message: &str) -> Result<(), String> {
        Ok(())
    }
    fn exit(&mut self) -> Result<(), String> {
        Ok(())
    }
}
