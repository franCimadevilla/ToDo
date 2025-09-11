use rustyline::Editor;
use rustyline::history::DefaultHistory;

use crate::{service::manager::Manager, ui::cli_argument_parser::trait_cli_displayer::TraitCliDisplayer};
use crate::ui::console_ui::generic_console_displayer::GenericConsoleDisplayer;
use crate::ui::displayer::Displayer;
use crate::ui::menu_option::MenuOption;
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

impl TraitCliDisplayer for CliDisplayer {
    fn handle_add_task(&mut self, manager: &mut Manager) {
        let _ = self.inner.handle_add_task(manager);
    }

    fn handle_edit_task(&mut self, manager: &mut Manager) {
        let _ = self.inner.handle_edit_task(manager);
    }
}