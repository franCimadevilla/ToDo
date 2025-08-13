/// tests/integration_tests.rs

use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// Helper function to run the application in a temporary directory with given input and capture output.
/// This ensures the test case isolation by using a new todo_list.json for each test.
fn run_app_with_input(input: &str) -> io::Result<String> {
    
    let temp_dir = TempDir::new()?;
    let prev_dir = env::current_dir()?;
    env::set_current_dir(&temp_dir)?;

    let exe = assert_cmd::cargo::cargo_bin("ToDo");
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    // Restore the original directory
    env::set_current_dir(&prev_dir)?;

    let mut full_output = String::from_utf8_lossy(&output.stdout).to_string();
    full_output.push_str(&String::from_utf8_lossy(&output.stderr));

    Ok(full_output)
}

/// Test the console menu display and invalid input handling
#[test]
fn test_console_menu_display_and_invalid_input() {
    let input = "invalid\n5\n"; // Invalid option, then exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("Welcome to the ToDo console application!"));
    assert!(output.contains("1. Add Task"));
    assert!(output.contains("2. List Tasks"));
    assert!(output.contains("3. Complete Task"));
    assert!(output.contains("4. Remove Task"));
    assert!(output.contains("5. Exit"));
    assert!(output.contains("[U] Undo"));
    assert!(output.contains("[R] Redo"));


    assert!(output.contains("Error: Invalid option, please try again."));
    assert!(output.contains("Exiting ToDo application... Goodbye!"));
}

/// Test adding a task with description and priority, then list to verify
#[test]
fn test_add_task() {
    let input = "1\nTest Task Description\n2\n2\n5\n"; // Add task, desc, priority Medium (2), list, exit
    let output = run_app_with_input(input).expect("Failed to run app");


    assert!(output.contains("You selected: Add Task"));
    assert!(output.contains("Enter task description:"));
    assert!(output.contains("Enter task priority number (1-High, 2-Medium, 3-Low):"));

    assert!(output.contains("Task added."));

    assert!(output.contains("You selected: List Tasks"));
    assert!(output.contains("ID: 1, Description: Test Task Description, Priority: Medium, Completed: false"));
}

/// Test adding a task with invalid priority (defaults to Low)
#[test]
fn test_add_task_default_priority() {
    let input = "1\nLow Priority Task\ninvalid\n2\n5\n"; // Add, desc, invalid priority, list, exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("Invalid priority, defaulting to Low."));
    assert!(output.contains("ID: 1, Description: Low Priority Task, Priority: Low, Completed: false"));
}

/// Test listing tasks when empty
#[test]
fn test_list_tasks_empty() {
    let input = "2\n5\n"; // List, exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("You selected: List Tasks"));
    assert!(!output.contains("ID:"));
    assert!(output.contains("No tasks in the list."));
}

/// Test listing tasks with multiple entries
#[test]
fn test_list_tasks_with_entries() {
    let input = "1\nTask1\n1\n1\nTask2\n3\n2\n5\n"; // Add Task1 High, Add Task2 Low, List, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("ID: 1, Description: Task1, Priority: High, Completed: false"));
    assert!(output.contains("ID: 2, Description: Task2, Priority: Low, Completed: false"));
}

/// Test completing a task (toggle status)
#[test]
fn test_complete_task_success() {
    let input = "1\nTask to Complete\n1\n3\n1\n5\n"; // Add task High, Complete with ID 1, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("You selected: Complete Task"));
    assert!(output.contains("Enter task ID to complete:"));
    assert!(output.contains("Task with ID 1 marked as completed.")); 

}

/// Test completing a non-existent task
#[test]
fn test_complete_task_not_found() {
    let input = "3\n999\n5\n"; // Complete with invalid ID 999, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("Task with ID 999 not found."));
}

/// Test deleting a task
#[test]
fn test_delete_task_success() {
    let input = "1\nTask to Delete\n1\n4\n1\n5\n"; // Add task, Delete with ID 1, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("You selected: Remove Task"));
    assert!(output.contains("Enter task ID to remove:"));
    assert!(output.contains("Task with ID 1 removed.")); 

}

/// Test deleting a non-existent task
#[test]
fn test_delete_task_not_found() {
    let input = "4\n999\n5\n"; // Remove with invalid ID 999, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("Task with ID 999 not found."));
}

/// Test Undo after adding a task
#[test]
fn test_undo_after_add() {
    let input = "1\nTask to Undo\n1\nU\n2\n5\n"; // Add task, Undo, List (should be empty), Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(!output.contains("ID:")); 
    assert!(output.contains("Undo operation successful."));
}

/// Test Redo after Undo the adding of a task
#[test]
fn test_redo_after_undo() {
    let input = "1\nTask to Redo\n1\nU\nR\n2\n5\n"; // Add task, Undo, Redo, List, Exit
    let output = run_app_with_input(input).expect("Failed to run app");

    assert!(output.contains("Task added."));
    assert!(output.contains("Undo operation successful."));
    assert!(output.contains("Redo operation successful."));
    assert!(output.contains("Description: Task to Redo, Priority: High, Completed: false"));
}