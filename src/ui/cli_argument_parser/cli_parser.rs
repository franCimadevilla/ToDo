use clap::{Parser, Subcommand};
use crate::model::priority::Priority;
use crate::service::manager::{Manager, ManagerTrait};
use crate::ui::displayer_trait::Displayer;

#[derive(Parser)]
#[command(name = "ToDo", version = "1.0")]
#[command(author = "Francisco Cimadevilla")]
#[command(about = "Console line application that manages a list of tasks.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum CliCommand {
    Add {
        #[arg(short = 'd', long)]
        description: String,

        #[arg(short = 'p', long, value_enum, default_value_t = Priority::Low)]
        priority: Priority,
    },
    List {
        #[arg(short = 'p', long, value_enum, required = false)]
        priority: Option<Priority>,

        #[arg(short = 'c', long ="completed", required = false)]
        completed: Option<bool>,
    },
    #[command(alias = "toggle", aliases = ["check"])]
    ToggleStatus {
        #[arg(short = 'i', long)]
        id : String,
    },
    Remove {
        #[arg(short = 'i', long)]
        id : String,
    }
}

impl Cli {
    pub fn evaluate_command(&self, command : CliCommand, manager: &mut Manager, displayer : &mut dyn Displayer ) {
        match command {
            CliCommand::Add { description, priority } => {
                manager.add_task(description.as_ref(), &priority);
                displayer.notify("Task added successfully.")
                    .expect("Failed to notify addition of a task.");
            },
            CliCommand::List { priority, completed } => {
    
                let tasks = manager.get_tasks();
                let mut filtered_tasks = match priority {
                    None => tasks.iter().collect::<Vec<_>>(),
                    Some(p) => tasks.iter().filter(|task| task.priority == p).collect::<Vec<_>>(),
                };
                if let Some(completed) = completed {
                    filtered_tasks = filtered_tasks
                        .into_iter()
                        .filter(|task| task.completed == completed)
                        .collect();
                }
                if filtered_tasks.is_empty() {
                    let message = match (priority, completed) {
                        (Some(p), Some(c)) => format!("No tasks found with priority {:?} and completed = {}", p, c),
                        (Some(p), None) => format!("No tasks found with priority {:?}", p),
                        (None, Some(c)) => format!("No tasks found with completed = {}", c),
                        (None, None) => "No tasks found.".to_string(),
                    };
                    displayer.notify(&message).expect("Failed to notify no tasks found");
                } else {
                    let message = match (priority, completed) {
                        (Some(p), Some(c)) => format!("{} tasks found with priority {:?} and completed = {}", filtered_tasks.len(), p, c),
                        (Some(p), None) => format!("{} tasks found with priority {:?}", filtered_tasks.len(), p),
                        (None, Some(c)) => format!("{} tasks found with completed = {}", filtered_tasks.len(), c),
                        (None, None) => format!("{} tasks found", filtered_tasks.len()),
                    };
                    displayer.notify(&message).expect("Failed to notify tasks found");
                    for task in filtered_tasks {
                        displayer
                            .notify(&format!(
                                "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                                task.id, task.description, task.priority, task.completed
                            ))
                            .expect("Failed to notify task details");
                    }
                }
            },
            CliCommand::Remove { id } => {
                if manager.get_task(&id).is_none() {
                    displayer.notify(&format!("Error: Task with ID {} not found", id))
                        .expect("Failed to notify error for task removal");
                    return;
                } else {
                    manager.remove_task(id.as_ref());
                    displayer.notify("Task removed successfully.")
                        .expect("Failed to notify removal of a task.");
                }
            },
            CliCommand::ToggleStatus { id } => {
                if manager.get_task(&id).is_none() {
                    displayer.notify(&format!("Error: Task with ID {} not found", id))
                        .expect("Failed to notify error for toggling task status");
                    return;
                } else {
                    manager.toggle_task_status(id.as_ref());
                    displayer.notify("Task status toggled successfully.")
                        .expect("Failed to notify toggling of task status.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{model::priority::Priority, ui::console_ui::menu_options::MenuOption};
    
    #[test]
    fn test_parse_add_command() {
        let cli = Cli::parse_from(&["ToDo", "add", "-d", "Test task", "-p", "High"]);
        assert_eq!(
            cli.command,
            Some(CliCommand::Add {
                description: "Test task".to_string(),
                priority: Priority::High,
            })
        );

        // Test default priority
        let cli = Cli::parse_from(&["ToDo", "add", "-d", "Test task"]);
        assert_eq!(
            cli.command,
            Some(CliCommand::Add {
                description: "Test task".to_string(),
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
            Some(CliCommand::ToggleStatus { id: "1".to_string() })
        );

        // Test alias
        let cli = Cli::parse_from(&["ToDo", "toggle", "-i", "2"]);
        assert_eq!(
            cli.command,
            Some(CliCommand::ToggleStatus { id: "2".to_string() })
        );

        let cli = Cli::parse_from(&["ToDo", "check", "-i", "3"]);
        assert_eq!(
            cli.command,
            Some(CliCommand::ToggleStatus { id: "3".to_string() })
        );
    }

    #[test]
    fn test_parse_remove_command() {
        let cli = Cli::parse_from(&["ToDo", "remove", "-i", "1"]);
        assert_eq!(
            cli.command,
            Some(CliCommand::Remove { id: "1".to_string() })
        );
    }

    struct MockDisplayer {
        notifications: Vec<String>,
    }

    impl MockDisplayer {
        fn new() -> Self {
            MockDisplayer {
                notifications: Vec::new(),
            }
        }
        fn get_notifications(&self) -> Vec<String> {
            self.notifications.clone()
        }
    }

    impl Displayer for MockDisplayer {
        fn new() -> Self {
            MockDisplayer::new()
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

    #[test]
        fn test_evaluate_add_command() {
            let mut manager = Manager::new(Box::new(MockDisplayer::new()));
            let mut displayer = MockDisplayer::new();
            let cli = Cli {
                command: Some(CliCommand::Add {
                    description: "Test task".to_string(),
                    priority: Priority::High,
                }),
            };
            let command = cli.command.as_ref().expect("Error during test").clone();
            cli.evaluate_command(command, &mut manager, &mut displayer);
            assert_eq!(manager.get_tasks().len(), 1);
            assert_eq!(manager.get_tasks()[0].description, "Test task");
            assert_eq!(manager.get_tasks()[0].priority, Priority::High);
            assert_eq!(displayer.get_notifications(), vec!["Task added successfully."]);
        }

    #[test]
    fn test_evaluate_list_command_empty() {
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        let mut displayer = MockDisplayer::new();
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
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        manager.add_task("Task 2".as_ref(), &Priority::High);
        let mut displayer = MockDisplayer::new();
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
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        manager.add_task("Task 2".as_ref(), &Priority::High);
        let mut displayer = MockDisplayer::new();
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
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        manager.toggle_task_status("1".as_ref());
        manager.add_task("Task 2".as_ref(), &Priority::High);
        let mut displayer = MockDisplayer::new();
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
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        manager.toggle_task_status("1".as_ref());
        manager.add_task("Task 2".as_ref(), &Priority::Low);
        let mut displayer = MockDisplayer::new();
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
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        let mut displayer = MockDisplayer::new();
        let cli = Cli {
            command: Some(CliCommand::ToggleStatus { id: "1".to_string() }),
        };
        let command = cli.command.as_ref().expect("Error during test").clone();
        cli.evaluate_command(command, &mut manager, &mut displayer);
        assert_eq!(manager.get_tasks()[0].completed, true);
        assert_eq!(displayer.notifications, vec!["Task status toggled successfully."]);
    }

    #[test]
    fn test_evaluate_toggle_status_error() {
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        let mut displayer = MockDisplayer::new();
        let cli = Cli {
            command: Some(CliCommand::ToggleStatus { id: "999".to_string() }),
        };
        let command = cli.command.as_ref().expect("Error during test").clone();
        cli.evaluate_command(command, &mut manager, &mut displayer);
        assert_eq!(displayer.notifications, vec!["Error: Task with ID 999 not found"]);
    }

    #[test]
    fn test_evaluate_remove_success() {
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        manager.add_task("Task 1".as_ref(), &Priority::Low);
        let mut displayer = MockDisplayer::new();
        let cli = Cli {
            command: Some(CliCommand::Remove { id: "1".to_string() }),
        };
        let command = cli.command.as_ref().expect("Error during test").clone();
        cli.evaluate_command(command, &mut manager, &mut displayer);
        assert_eq!(manager.get_tasks().len(), 0);
        assert_eq!(displayer.notifications, vec!["Task removed successfully."]);
    }

    #[test]
    fn test_evaluate_remove_error() {
        let mut manager = Manager::new(Box::new(MockDisplayer::new()));
        let mut displayer = MockDisplayer::new();
        let cli = Cli {
            command: Some(CliCommand::Remove { id: "999".to_string() }),
        };
        let command = cli.command.as_ref().expect("Error during test").clone();
        cli.evaluate_command(command, &mut manager, &mut displayer);
        assert_eq!(displayer.notifications, vec!["Error: Task with ID 999 not found"]);
    }
}