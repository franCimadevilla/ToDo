#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuOption {
    AddTask,
    ListTasks,
    CompleteTask,
    RemoveTask,
    Exit,
    Undo,
    Redo,
}