use crate::ui::console_ui::menu_options::MenuOption;

pub trait Displayer {
    fn new() -> Self;
    fn run(&mut self);
    fn display(&self) -> Result<MenuOption, String>;
    fn exit(&self);
}