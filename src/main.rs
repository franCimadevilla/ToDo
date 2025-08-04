use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::console_displayer::ConsoleDisplayer;

pub mod ui;
pub mod service;
pub mod model;

fn main() {
    let mut displayer = ConsoleDisplayer::new();
    displayer.run();
}
