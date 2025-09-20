use crate::{service::manager::Manager, ui::displayer::Displayer};
pub trait TraitCliDisplayer: Displayer {
    fn handle_add_task(&mut self, manager: &mut Manager);
    fn handle_edit_task(&mut self, manager: &mut Manager);
}