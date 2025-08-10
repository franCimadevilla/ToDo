use crate::ui::console_ui::generic_console_displayer::GenericConsoleDisplayer;
use std::io::{BufReader, Stdin, Stdout};
use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::Manager;
use crate::ui::displayer_trait::Displayer;

pub struct CliDisplayer {
    inner : GenericConsoleDisplayer<BufReader<Stdin>, Stdout>,
}

impl CliDisplayer {
    pub fn new() -> Self {
        CliDisplayer {
            inner: GenericConsoleDisplayer::new(
                BufReader::new(std::io::stdin()),
                std::io::stdout(),
            ),
        }
    }
}

impl Displayer for CliDisplayer {
    fn new() -> Self {
        CliDisplayer::new()
    }

    fn run(&mut self, _manager: &mut Manager) { () }

    fn display(&mut self) -> Result<MenuOption, String> { Ok(MenuOption::Exit) }

    fn notify(&mut self, message: &str) -> Result<(), String> { self.inner.notify(message) }

    fn exit(&mut self) -> Result<(), String> { self.inner.exit() }
}