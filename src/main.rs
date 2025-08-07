use crate::ui::displayer_trait::Displayer;
use crate::ui::console_ui::console_displayer::ConsoleDisplayer;
use crate::service::manager::{Manager, ManagerTrait};

pub mod ui;
pub mod service;
pub mod model;

fn main() {
    let displayer: Box<dyn Displayer> = Box::new(ConsoleDisplayer::new());
    let mut manager = Manager::new(displayer);
    manager.run();
}
