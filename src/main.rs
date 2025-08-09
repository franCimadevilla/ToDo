use clap::Parser;
use to_do::service::manager::{Manager, ManagerTrait};
use to_do::ui::console_ui::console_displayer::ConsoleDisplayer;
use to_do::ui::displayer_trait::Displayer;
use to_do::ui::cli_argument_parser::cli_parser::{Cli};
use to_do::ui::cli_argument_parser::cli_displayer::CliDisplayer;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(command) => {
            let cli_displayer: Box<dyn Displayer> = Box::new(CliDisplayer::new());
            let mut manager = Manager::new(Box::new(CliDisplayer::new()));
            manager.run();
            cli.evaluate_command(command.clone(), &mut manager, cli_displayer);
        }
        None => {
            let displayer: Box<dyn Displayer> = Box::new(ConsoleDisplayer::new());
            let mut manager = Manager::new(displayer);
            manager.run();
        }
    }
}
