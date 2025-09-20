use clap::Parser;
use to_do::ui::cli_argument_parser::cli_parser::{Cli, CliCommand};
use to_do::ui::cli_argument_parser::trait_cli_displayer::TraitCliDisplayer;
use to_do::{model::priority::Priority};
use to_do::ui::{displayer::Displayer, menu_option::MenuOption};
use to_do::service::manager::{Manager, ManagerTrait};
    
struct StackMockDisplayer {
    notifications: Vec<String>,
}

impl StackMockDisplayer {
    fn new() -> Self {
        StackMockDisplayer {
            notifications: Vec::new(),
        }
    }
    fn get_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }
}

impl Displayer for StackMockDisplayer {
    fn new() -> Self {
        StackMockDisplayer::new()
    }
    fn run(&mut self, _manager: &mut Manager) {}
    fn display(&mut self) -> Result<MenuOption, String> {
        Ok(MenuOption::Exit)
    }
    fn notify(&mut self, message: &str) -> Result<(), String> {
        self.notifications.push(message.to_string());
        Ok(())
    }
    fn exit(&mut self) -> Result<(), String> {
        Ok(())
    }
}


impl TraitCliDisplayer for StackMockDisplayer {
    
    fn handle_add_task(&mut self, _manager: &mut Manager) {
        self.notifications.push("You selected: Add Task".into());
    }

    fn handle_edit_task(&mut self, _manager: &mut Manager) {
        self.notifications.push("You selected: Edit Task".into());
    }
}

impl Clone for StackMockDisplayer {
    fn clone(&self) -> Self {
        StackMockDisplayer { notifications: self.notifications.clone() }
    }
}


#[test]
fn test_parse_add_command() {
    let cli = Cli::parse_from(&["ToDo", "add", "-d", "Test task", "-p", "High"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::Add {
            description: Some("Test task".to_string()),
            priority: Priority::High,
        })
    );

    // Test default priority
    let cli = Cli::parse_from(&["ToDo", "add", "-d", "Test task"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::Add {
            description: Some("Test task".to_string()),
            priority: Priority::Low,
        })
    );
}

#[test]
fn test_parse_list_command() {
    // No filters
    let cli = Cli::parse_from(&["ToDo", "list"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::List {
            priority: None,
            completed: None,
        })
    );

    // With priority
    let cli = Cli::parse_from(&["ToDo", "list", "-p", "Medium"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::List {
            priority: Some(Priority::Medium),
            completed: None,
        })
    );

    // With completed
    let cli = Cli::parse_from(&["ToDo", "list", "-c", "true"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::List {
            priority: None,
            completed: Some(true),
        })
    );

    // With both
    let cli = Cli::parse_from(&["ToDo", "list", "-p", "Low", "-c", "false"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::List {
            priority: Some(Priority::Low),
            completed: Some(false),
        })
    );
}

#[test]
fn test_parse_toggle_status_command() {
    let cli = Cli::parse_from(&["ToDo", "toggle-status", "-i", "1"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::ToggleStatus {
            id: "1".to_string()
        })
    );

    // Test alias
    let cli = Cli::parse_from(&["ToDo", "toggle", "-i", "2"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::ToggleStatus {
            id: "2".to_string()
        })
    );

    let cli = Cli::parse_from(&["ToDo", "check", "-i", "3"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::ToggleStatus {
            id: "3".to_string()
        })
    );
}

#[test]
fn test_parse_remove_command() {
    let cli = Cli::parse_from(&["ToDo", "remove", "-i", "1"]);
    assert_eq!(
        cli.command,
        Some(CliCommand::Remove {
            id: "1".to_string()
        })
    );
}

#[test]
fn test_evaluate_add_command() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::Add {
            description: Some("Test task".to_string()),
            priority: Priority::High,
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(manager.get_tasks().len(), 1);
    assert_eq!(manager.get_tasks()[0].description, "Test task");
    assert_eq!(manager.get_tasks()[0].priority, Priority::High);
    assert_eq!(
        displayer.get_notifications(),
        vec!["Task added successfully."]
    );
}

#[test]
fn test_evaluate_list_command_empty() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::List {
            priority: None,
            completed: None,
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(displayer.notifications, vec!["No tasks found."]);
}

#[test]
fn test_evaluate_list_command_with_tasks() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    manager.add_task("Task 2".as_ref(), &Priority::High);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::List {
            priority: None,
            completed: None,
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec![
            "2 tasks found",
            "ID: 1, Description: Task 1, Priority: Low, Completed: false",
            "ID: 2, Description: Task 2, Priority: High, Completed: false",
        ]
    );
}

#[test]
fn test_evaluate_list_command_filtered_priority() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    manager.add_task("Task 2".as_ref(), &Priority::High);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::List {
            priority: Some(Priority::Low),
            completed: None,
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec![
            "1 tasks found with priority Low",
            "ID: 1, Description: Task 1, Priority: Low, Completed: false",
        ]
    );
}

#[test]
fn test_evaluate_list_command_filtered_completed() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    manager.toggle_task_status("1".as_ref());
    manager.add_task("Task 2".as_ref(), &Priority::High);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::List {
            priority: None,
            completed: Some(true),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec![
            "1 tasks found with completed = true",
            "ID: 1, Description: Task 1, Priority: Low, Completed: true",
        ]
    );
}

#[test]
fn test_evaluate_list_command_filtered_both() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    manager.toggle_task_status("1".as_ref());
    manager.add_task("Task 2".as_ref(), &Priority::Low);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::List {
            priority: Some(Priority::Low),
            completed: Some(true),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec![
            "1 tasks found with priority Low and completed = true",
            "ID: 1, Description: Task 1, Priority: Low, Completed: true",
        ]
    );
}

#[test]
fn test_evaluate_toggle_status_success() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::ToggleStatus {
            id: "1".to_string(),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(manager.get_tasks()[0].completed, true);
    assert_eq!(
        displayer.notifications,
        vec!["Task status toggled successfully."]
    );
}

#[test]
fn test_evaluate_toggle_status_error() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::ToggleStatus {
            id: "999".to_string(),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec!["Error: Task with ID: 999 not found"]
    );
}

#[test]
fn test_evaluate_remove_success() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::Remove {
            id: "1".to_string(),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(manager.get_tasks().len(), 0);
    assert_eq!(displayer.notifications, vec!["Task removed successfully."]);
}

#[test]
fn test_evaluate_remove_error() {
    let mut manager = Manager::new(Box::new(StackMockDisplayer::new()));
    let mut displayer = StackMockDisplayer::new();
    let cli = Cli {
        command: Some(CliCommand::Remove {
            id: "999".to_string(),
        }),
    };
    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications,
        vec!["Error: Task with ID: 999 not found"]
    );
}

#[test]
fn test_evaluate_edit_pattern_success() {
    let mut displayer = StackMockDisplayer::new();
    let mut manager = Manager::new(Box::new(displayer.clone()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);

    let cli = Cli {
        command: None,
    };

    let command = 
        CliCommand::Edit { id: Some("1".to_string()), pattern: Some("Task".to_string()),
            replace: Some("Tarea".to_string()), priority: None  };
    let command_list= CliCommand::List { priority: None, completed: None };
    
    cli.evaluate_command(command, &mut manager, &mut displayer);
    cli.evaluate_command(command_list, &mut manager, &mut displayer);
    assert!(
        displayer.notifications.clone()
        .contains(&"ID: 1, Description: Tarea 1, Priority: Low, Completed: false".to_string())
    );
}

#[test]
fn test_evaluate_edit_pattern_task_not_found() {
    let mut displayer = StackMockDisplayer::new();
    let mut manager = Manager::new(Box::new(displayer.clone()));

    let cli = Cli {
        command: None,
    };

    let command = 
        CliCommand::Edit { id: Some("1".to_string()), pattern: Some("Task".to_string()),
            replace: Some("Tarea".to_string()), priority: None  };
    
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert!(
        displayer.notifications.clone()
        .contains(&"Error: Task with ID: 1 not found".to_string())
    );
}

#[test]
fn test_evaluate_edit_pattern_no_match() {
    let mut displayer = StackMockDisplayer::new();
    let mut manager = Manager::new(Box::new(displayer.clone()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);

    let cli = Cli {
        command: None,
    };

    let command = 
        CliCommand::Edit { id: Some("1".to_string()), pattern: Some("Tarea".to_string()),
            replace: Some("Tarea".to_string()), priority: None  };
    let command_list= CliCommand::List { priority: None, completed: None };
    
    cli.evaluate_command(command, &mut manager, &mut displayer);
    cli.evaluate_command(command_list, &mut manager, &mut displayer);
    assert!(
        displayer.notifications.clone()
        .contains(&"ID: 1, Description: Task 1, Priority: Low, Completed: false".to_string())
    );
}

#[test]
fn test_evaluate_edit_pattern_none_replace_not_blank() {
    let mut displayer = StackMockDisplayer::new();
    let mut manager = Manager::new(Box::new(displayer.clone()));
    manager.add_task("Task 1".as_ref(), &Priority::Low);

    let cli = Cli {
        command: None,
    };

    let command = 
        CliCommand::Edit { id: Some("1".to_string()), pattern: None,
            replace: Some("Tarea".to_string()), priority: None  };
    let command_list= CliCommand::List { priority: None, completed: None };
    
    cli.evaluate_command(command, &mut manager, &mut displayer);
    cli.evaluate_command(command_list, &mut manager, &mut displayer);
    assert!(
        displayer.notifications.clone()
        .contains(&"Error: --pattern and --replace must both be provided or both omitted.".to_string())
    );
}

#[test]
fn test_evaluate_edit_alt_display() {
    let mut displayer = StackMockDisplayer::new();
    let mut manager = Manager::new(Box::new(displayer.clone()));
    let cli = Cli {
        command: Some(CliCommand::Edit {
            id: None,
            pattern: None,
            replace: None,
            priority: None
        }),
    };

    let command = cli.command.as_ref().expect("Error during test").clone();
    cli.evaluate_command(command, &mut manager, &mut displayer);
    assert_eq!(
        displayer.notifications.clone(),
        vec!["You selected: Edit Task"]
    );
}