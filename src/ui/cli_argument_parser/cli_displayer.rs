use rustyline::Editor;
use rustyline::history::DefaultHistory;

use crate::service::manager::Manager;
use crate::service::menu_option::MenuOption;
use crate::ui::console_ui::generic_console_displayer::GenericConsoleDisplayer;
use crate::ui::displayer::Displayer;
use std::io::{BufReader, Stdin, Stdout};

pub struct CliDisplayer {
    inner: GenericConsoleDisplayer<BufReader<Stdin>, Stdout, Editor<(), DefaultHistory>>,
}

impl CliDisplayer {
    pub fn new() -> Self {
        CliDisplayer {
            inner: GenericConsoleDisplayer::new(
                BufReader::new(std::io::stdin()),
                std::io::stdout(),
                Editor::<(), DefaultHistory>::new().expect("Failed to create editor"),
            ),
        }
    }
}

impl Displayer for CliDisplayer {
    fn new() -> Self {
        CliDisplayer::new()
    }

    fn run(&mut self, _manager: &mut Manager) {
        ()
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        Ok(MenuOption::Exit)
    }

    fn notify(&mut self, message: &str) -> Result<(), String> {
        self.inner.notify(message)
    }

    fn exit(&mut self) -> Result<(), String> {
        self.inner.exit()
    }
}
