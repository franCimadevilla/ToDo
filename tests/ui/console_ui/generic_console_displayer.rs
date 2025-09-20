use std::io::Cursor;
use to_do::model::priority::Priority;
use to_do::service::manager::{Manager, ManagerTrait};
use to_do::ui::{
    console_ui::{
        generic_console_displayer::GenericConsoleDisplayer, mock_displayer::MockDisplayer,
    },
    displayer::Displayer,
    line_editor::MockLineEditor,
    menu_option::MenuOption,
};

fn create_manager_with_tasks() -> Manager {
    let displayer: Box<dyn Displayer> = Box::new(MockDisplayer);
    let mut manager = Manager::new(displayer);
    manager.add_task("Test Task 1".as_ref(), &Priority::High);
    manager.add_task("Test Task 2".as_ref(), &Priority::Medium);
    manager
}

fn create_displayer_mocked_editor(
    input: Cursor<String>,
    output: Cursor<Vec<u8>>,
    editor_vec: Vec<String>,
) -> GenericConsoleDisplayer<Cursor<String>, Cursor<Vec<u8>>, MockLineEditor> {
    let editor = MockLineEditor::new(editor_vec);
    GenericConsoleDisplayer::new(input, output, editor)
}

#[test]
fn test_notify() {
    let input = Cursor::new("".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    displayer.notify("Test message").expect("Notify failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert_eq!(output, "[Test message]\n");
}

#[test]
fn test_exit() {
    let input = Cursor::new("".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    displayer.exit().expect("Exit failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert_eq!(output, "Exiting ToDo application... Goodbye!\n");
}

#[test]
fn test_display_add_task() {
    let input_vec = vec![MenuOption::get_input_key(&MenuOption::AddTask).to_string()];

    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());

    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());

    let result = displayer.display().expect("Display failed");
    assert_eq!(result, MenuOption::AddTask);

    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("ToDo Operations:"));
    assert!(output.contains("1. Add Task"));
    assert!(output.contains("Enter your choice (1-5):"));
}

#[test]
fn test_display_invalid_option() {
    let input = Cursor::new("invalid\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let result = displayer.display();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "The option: invalid is invalid, please try again."
    );
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("ToDo Operations:"));
}

#[test]
fn test_handle_add_task() {
    let input = Cursor::new("Test Task\n2\n".to_string());
    let output = Cursor::new(Vec::new());

    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();

    displayer
        .handle_add_task(&mut manager)
        .expect("Add task failed");

    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("You selected: Add Task"));
    assert!(output.contains("Enter task description:"));
    assert!(output.contains("Enter task priority number"));
    let tasks = manager.get_tasks();
    assert!(
        tasks
            .iter()
            .any(|t| t.description == "Test Task" && t.priority == Priority::Medium)
    );
}

#[test]
fn test_handle_add_task_invalid_priority() {
    let input = Cursor::new("Test Task\ninvalid\n2\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    displayer
        .handle_add_task(&mut manager)
        .expect("Add task failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Invalid priority, please type again a valid one."));
    let tasks = manager.get_tasks();
    assert!(
        tasks
            .iter()
            .any(|t| t.description == "Test Task" && t.priority == Priority::Medium)
    );
}

#[test]
fn test_handle_list_tasks() {
    let input = Cursor::new("".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let manager = create_manager_with_tasks();
    displayer
        .handle_list_tasks(&manager)
        .expect("List tasks failed");
    let output = String::from_utf8(displayer.output.into_inner())
        .expect("Failed to convert output to string");
    assert!(output.contains("You selected: List Tasks"));
    assert!(output.contains("Test Task 1"));
    assert!(output.contains("Test Task 2"));
}

#[test]
fn test_handle_toggle_task_success() {
    let input = Cursor::new("1\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    displayer
        .handle_toggle_task(&mut manager)
        .expect("Complete task failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("You selected: Complete Task"));
    assert!(output.contains("Task with ID 1 marked as completed"));
    let tasks = manager.get_tasks();
    assert!(tasks.iter().find(|t| t.id == "1").unwrap().completed);
}

#[test]
fn test_handle_toggle_task_not_found() {
    let input = Cursor::new("999\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    displayer
        .handle_toggle_task(&mut manager)
        .expect("Complete task failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Task with ID 999 not found"));
}

#[test]
fn test_handle_remove_task_success() {
    let input = Cursor::new("1\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    displayer
        .handle_remove_task(&mut manager)
        .expect("Remove task failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("You selected: Remove Task"));
    assert!(output.contains("Task with ID 1 removed"));
    let tasks = manager.get_tasks();
    assert!(!tasks.iter().any(|t| t.id == "1"));
}

#[test]
fn test_handle_remove_task_not_found() {
    let input = Cursor::new("999\n".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    displayer
        .handle_remove_task(&mut manager)
        .expect("Remove task failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Task with ID 999 not found"));
}

#[test]
fn test_handle_undo() {
    let input = Cursor::new("".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    manager.remove_task("1".as_ref());
    displayer.handle_undo(&mut manager).expect("Undo failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    let tasks = manager.get_tasks();
    assert!(tasks.iter().any(|t| t.id == "1"));
    assert!(!output.contains("Undo failed"));
}

#[test]
fn test_handle_redo() {
    let input = Cursor::new("".to_string());
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();
    let id_to_remove = manager.get_tasks()[0].id.clone();
    manager.remove_task(id_to_remove.as_ref());
    manager.undo().expect("Undo failed");
    displayer.handle_redo(&mut manager).expect("Redo failed");
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Redo operation successful"));
    let tasks = manager.get_tasks();
    assert!(!tasks.iter().any(|t| t.id == "1"));
}

#[test]
fn test_run_add_and_exit() {
    let input_vec = vec![
        MenuOption::get_input_key(&MenuOption::AddTask).to_string(),
        "Test Task".to_string(),
        "2".to_string(),
        MenuOption::get_input_key(&MenuOption::Exit).to_string(),
    ];
    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();

    displayer.run(&mut manager);

    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Welcome to the ToDo console application!"));
    assert!(output.contains("You selected: Add Task"));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
    let tasks = manager.get_tasks();
    assert!(
        tasks
            .iter()
            .any(|t| t.description == "Test Task" && t.priority == Priority::Medium)
    );
}

#[test]
fn test_run_invalid_input() {
    let input_vec = vec![
        "invalid".to_string(),
        MenuOption::get_input_key(&MenuOption::Exit).to_string(),
    ];
    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();

    displayer.run(&mut manager);

    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("The option: invalid is invalid, please try again"));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
}

#[test]
fn test_console_displayer_run() {
    let input_vec = vec![
        MenuOption::get_input_key(&MenuOption::AddTask).to_string(),
        "Test Task".to_string(),
        "2".to_string(),
        MenuOption::get_input_key(&MenuOption::Exit).to_string(),
    ];
    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());
    let mut displayer = create_displayer_mocked_editor(input, output, Vec::new());
    let mut manager = create_manager_with_tasks();

    displayer.run(&mut manager);

    let output = String::from_utf8(displayer.output.into_inner()).unwrap();
    assert!(output.contains("Welcome to the ToDo console application!"));
    assert!(output.contains("You selected: Add Task"));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
    let tasks = manager.get_tasks();
    assert!(
        tasks
            .iter()
            .any(|t| t.description == "Test Task" && t.priority == Priority::Medium)
    );
}

#[test]
fn test_handle_edit_task_no_change() {
    let input_vec = vec![
        MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
        MenuOption::get_input_key(&MenuOption::EditTask).to_string(),
        "1".into(), //ID to edit
        MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
        MenuOption::get_input_key(&MenuOption::Exit).to_string(),
    ];
    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());
    let editor = MockLineEditor::new(vec![
        //Edited fields inputed for editor
        "Test Task 1".into(),
        "1".into(),
    ]);
    let mut displayer = GenericConsoleDisplayer::new(input, output, editor);
    let mut manager = create_manager_with_tasks();
    displayer.run(&mut manager);
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();

    assert!(output.contains("Welcome to the ToDo console application!"));
    assert!(output.contains("Description: Test Task 1, Priority: High, Completed: false"));
    assert!(output.contains("You selected: Edit Task"));
    assert!(output.contains("Description: Test Task 1, Priority: High, Completed: false"));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
}

#[test]
fn test_handle_edit_task_change_fields() {
    let input_vec = vec![
        MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
        MenuOption::get_input_key(&MenuOption::EditTask).to_string(),
        "1".into(), //ID to edit
        MenuOption::get_input_key(&MenuOption::ListTasks).to_string(),
        MenuOption::get_input_key(&MenuOption::Exit).to_string(),
    ];
    let mut input = input_vec.join("\n");
    input.push_str("\n");

    let input = Cursor::new(input);
    let output = Cursor::new(Vec::new());
    let editor = MockLineEditor::new(vec![
        //Edited fields inputed for editor
        "New Description".into(),
        "3".into(),
    ]);
    let mut displayer = GenericConsoleDisplayer::new(input, output, editor);
    let mut manager = create_manager_with_tasks();
    displayer.run(&mut manager);
    let output = String::from_utf8(displayer.output.into_inner()).unwrap();

    assert!(output.contains("Welcome to the ToDo console application!"));
    assert!(output.contains("Description: Test Task 1, Priority: High, Completed: false"));
    assert!(output.contains("You selected: Edit Task"));
    assert!(output.contains("Description: New Description, Priority: Low, Completed: false"));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
}
