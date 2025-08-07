use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::{Manager};

pub trait Displayer : Send + Sync{
    fn new() -> Self 
    where Self: Sized;

    fn run(&mut self, manager : &mut Manager);
    fn display(&self) -> Result<MenuOption, String>;
    fn notify(&self, message:&str);
    fn exit(&self);
}