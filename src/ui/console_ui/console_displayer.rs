use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::menu_options::MenuOption;
use crate::service::manager::{Manager};
use std::io::{BufReader, Stdin, Stdout};
use crate::ui::console_ui::generic_console_displayer::GenericConsoleDisplayer;

/// ConsoleDisplayer for production, wrapping GenericConsoleDisplayer with Stdin/Stdout.
pub struct ConsoleDisplayer {
    inner: GenericConsoleDisplayer<BufReader<Stdin>, Stdout>,
}

impl ConsoleDisplayer {
    pub fn new() -> Self {
        ConsoleDisplayer {
            inner: GenericConsoleDisplayer::new(
                BufReader::new(std::io::stdin()),
                std::io::stdout(),
            ),
        }
    }
}

impl Displayer for ConsoleDisplayer {
    fn new() -> Self {
        ConsoleDisplayer::new()
    }

    fn run(&mut self, manager: &mut Manager) {
        self.inner.run(manager)
    }

    fn display(&mut self) -> Result<MenuOption, String> {
        self.inner.display()
    }

    fn notify(&mut self, message: &str) -> Result<(), String> {
        return self.inner.notify(message);
    
    }

    fn exit(&mut self) -> Result<(), String> {
        return self.inner.exit();
    }

}