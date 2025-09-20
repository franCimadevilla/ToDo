
use to_do::model::{todo_list::TodoList, priority::Priority};
use std::fs;

#[test]
fn test_new_todo_list() {
    let todo_list = TodoList::new();
    assert_eq!(todo_list.tasks.len(), 0);
    assert_eq!(todo_list.next_id, 1);
    assert_eq!(todo_list.file_name, "todo_list.json");
}

#[test]
fn test_add_task() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Test task".to_string(), Priority::High);
    assert_eq!(todo_list.tasks.len(), 1);
    assert_eq!(todo_list.next_id, 2);
    assert_eq!(todo_list.tasks[0].id, id);
    assert_eq!(todo_list.tasks[0].description, "Test task");
    assert_eq!(todo_list.tasks[0].priority, Priority::High);
    assert_eq!(todo_list.tasks[0].completed, false);
}

#[test]
fn test_complete_task() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Test task".to_string(), Priority::Medium);
    todo_list.toggle_task_status(id.clone());
    assert_eq!(todo_list.tasks[0].completed, true);
    // Toggle again to test flipping back
    todo_list.toggle_task_status(id.clone());
    assert_eq!(todo_list.tasks[0].completed, false);
}

#[test]
#[should_panic(expected = "IllegalArgument Error: Task with ID: notfound not found")]
fn test_complete_task_not_found() {
    let mut todo_list = TodoList::new();
    todo_list.toggle_task_status("notfound".to_string());
}

#[test]
fn test_remove_task() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Test task".to_string(), Priority::Low);
    todo_list.remove_task(id.clone());
    assert_eq!(todo_list.tasks.len(), 0);
}

#[test]
#[should_panic(expected = "IllegalArgument Error: Task with ID: notfound not found")]
fn test_remove_task_not_found() {
    let mut todo_list = TodoList::new();
    todo_list.remove_task("notfound".to_string());
}

#[test]
fn test_save_and_load() {
    let mut todo_list = TodoList::new();
    let test_file = "test_todo_list.json";
    todo_list.file_name = test_file.to_string();

    // Add a task and save
    todo_list.add_task("Test task".to_string(), Priority::High);
    todo_list.save();

    // Load into a new TodoList
    let mut new_todo_list = TodoList::new();
    new_todo_list.file_name = test_file.to_string();
    new_todo_list.load();

    assert_eq!(new_todo_list.tasks.len(), 1);
    assert_eq!(new_todo_list.tasks[0].description, "Test task");
    assert_eq!(new_todo_list.tasks[0].priority, Priority::High);

    // Clean up
    fs::remove_file(test_file).expect("Failed to clean up test file");
}

#[test]
fn test_try_load_no_file() {
    let mut todo_list = TodoList::new();
    let result = todo_list.load_from_file("no_exist.json");
    assert!(result.is_err());
}

#[test]
fn test_try_load_invalid_json() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("todo.json");
    std::fs::write(&file_path, "invalid json").unwrap();
    let mut todo_list = TodoList::new();
    let result = todo_list.load_from_file("todo.json");
    assert!(result.is_err());
}

#[test]
fn test_edit_task() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Test task".into(), Priority::High);
    let new_text = "Edited text".into();
    let new_priority = Priority::Low;
    todo_list.edit_task(id.as_ref(), (new_text, &new_priority));

    let edited_task = todo_list.get_task_mut(id.as_ref()).expect("Task not found");
    assert_eq!(edited_task.description, new_text.to_string())
}

#[test]
fn test_get_task_mut_exist() {
    let mut todo_list = TodoList::new();
    let id = todo_list.add_task("Test task".into(), Priority::High);

    let task = todo_list.get_task_mut(id.as_ref());
    assert!(task.is_some())
}

#[test]
fn test_task_mut_not_found() {
    let mut todo_list = TodoList::new();
    let id = "2".to_string();
    let task = todo_list.get_task_mut(id.as_ref());
    assert!(task.is_none())
}
