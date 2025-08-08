use to_do::service::manager::{Manager, ManagerTrait};
use to_do::ui::console_ui::console_displayer::ConsoleDisplayer;
use to_do::ui::displayer_trait::Displayer;


fn main() {
    let displayer: Box<dyn Displayer> = Box::new(ConsoleDisplayer::new());
    let mut manager = Manager::new(displayer);
    manager.run();
}
