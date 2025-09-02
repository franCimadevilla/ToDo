use clap::Parser;
use to_do::service::manager::{Manager, ManagerTrait};
use to_do::ui::cli_argument_parser::cli_displayer::CliDisplayer;
use to_do::ui::cli_argument_parser::cli_parser::Cli;
use to_do::ui::console_ui::console_displayer::ConsoleDisplayer;
use to_do::ui::displayer::Displayer;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(command) => {
            let mut cli_displayer = CliDisplayer::new();
            let mut manager = Manager::new(Box::new(CliDisplayer::new()));
            manager.run();
            cli.evaluate_command(command.clone(), &mut manager, &mut cli_displayer);
        }
        None => {
            let displayer: Box<dyn Displayer> = Box::new(ConsoleDisplayer::new());
            let mut manager = Manager::new(displayer);
            manager.run();
        }
    }
}
