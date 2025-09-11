use crate::model::priority::Priority;
use crate::service::manager::{Manager, ManagerTrait};
use crate::ui::cli_argument_parser::trait_cli_displayer::TraitCliDisplayer;
use clap::{Parser, Subcommand};

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
    #[command(about = "Add a new task")]
    Add {
        #[arg(short = 'd', long = "desc", help = "Description of the new task")]
        description: Option<String>,

        #[arg(short = 'p', long = "pri", value_enum, default_value_t = Priority::Low, help = "Priority of the new task")]
        priority: Priority,
    },

    #[command(about = "List all existing tasks")]
    List {
        #[arg(
            short = 'p',
            long = "pri",
            value_enum,
            required = false,
            help = "Filter the list of tasks by priority"
        )]
        priority: Option<Priority>,

        #[arg(
            short = 'c',
            long = "com",
            required = false,
            help = "Filter the list of tasks by completed status"
        )]
        completed: Option<bool>,
    },

    #[command(alias = "toggle", aliases = ["check"], about = "Change the completed/uncompleted status of a task")]
    ToggleStatus {
        #[arg(short = 'i', long = "id")]
        id: String,
    },

    #[command(about = "Remove an existing task")]
    Remove {
        #[arg(short = 'i', long = "id")]
        id: String,
    },

    #[command(about = "Edit an existing task")]
    Edit {
        #[arg(short = 'i', long = "id", help = "ID of the TODO item to edit")]
        id: Option<String>,

        #[arg(long = "pat", help = "Pattern to search in the TODO description")]
        pattern: Option<String>,

        #[arg(long = "rep", help = "Replacement for the matched pattern")]
        replace: Option<String>,

        #[arg(
            long = "pri",
            value_enum,
            help = "Priority of the TODO item (Low, Medium, High)"
        )]
        priority: Option<Priority>,
    },
}

impl Cli {
    pub fn evaluate_command(
        &self,
        command: CliCommand,
        manager: &mut Manager,
        displayer: &mut dyn TraitCliDisplayer,
    ) {
        match command {
            CliCommand::Add {
                description,
                priority,
            } => match description {
                Some(desc) => {
                    manager.add_task(desc.as_ref(), &priority);
                    displayer
                        .notify("Task added successfully.")
                        .expect("Failed to notify addition of a task.");
                }
                None => {
                    displayer.handle_add_task(manager);
                }
            },
            CliCommand::List {
                priority,
                completed,
            } => {
                let tasks = manager.get_tasks();
                let mut filtered_tasks = match priority {
                    None => tasks.iter().collect::<Vec<_>>(),
                    Some(p) => tasks
                        .iter()
                        .filter(|task| task.priority == p)
                        .collect::<Vec<_>>(),
                };
                if let Some(completed) = completed {
                    filtered_tasks = filtered_tasks
                        .into_iter()
                        .filter(|task| task.completed == completed)
                        .collect();
                }
                if filtered_tasks.is_empty() {
                    let message = match (priority, completed) {
                        (Some(p), Some(c)) => {
                            format!("No tasks found with priority {:?} and completed = {}", p, c)
                        }
                        (Some(p), None) => format!("No tasks found with priority {:?}", p),
                        (None, Some(c)) => format!("No tasks found with completed = {}", c),
                        (None, None) => "No tasks found.".to_string(),
                    };
                    displayer
                        .notify(&message)
                        .expect("Failed to notify no tasks found");
                } else {
                    let message = match (priority, completed) {
                        (Some(p), Some(c)) => format!(
                            "{} tasks found with priority {:?} and completed = {}",
                            filtered_tasks.len(),
                            p,
                            c
                        ),
                        (Some(p), None) => {
                            format!("{} tasks found with priority {:?}", filtered_tasks.len(), p)
                        }
                        (None, Some(c)) => format!(
                            "{} tasks found with completed = {}",
                            filtered_tasks.len(),
                            c
                        ),
                        (None, None) => format!("{} tasks found", filtered_tasks.len()),
                    };
                    displayer
                        .notify(&message)
                        .expect("Failed to notify tasks found");
                    for task in filtered_tasks {
                        displayer
                            .notify(&format!(
                                "ID: {}, Description: {}, Priority: {:?}, Completed: {}",
                                task.id, task.description, task.priority, task.completed
                            ))
                            .expect("Failed to notify task details");
                    }
                }
            }
            CliCommand::Remove { id } => {
                if Cli::is_task(id.as_ref(), manager, displayer) {
                    manager.remove_task(id.as_ref());
                    displayer
                        .notify("Task removed successfully.")
                        .expect("Failed to notify removal of a task.");
                }
            }
            CliCommand::ToggleStatus { id } => {
                if Cli::is_task(id.as_ref(), manager, displayer) {
                    manager.toggle_task_status(id.as_ref());
                    displayer
                        .notify("Task status toggled successfully.")
                        .expect("Failed to notify toggling of task status.");
                }
            }
            CliCommand::Edit {
                id,
                pattern,
                replace,
                priority,
            } => match id {
                Some(id) => {
                    if Cli::is_task(id.as_ref(), manager, displayer) {
                        let task = manager
                            .get_task(id.as_ref())
                            .expect("Failed to get the task when editing");
                        if pattern.is_some() != replace.is_some() {
                            displayer
                                    .notify("Error: --pattern and --replace must both be provided or both omitted.")
                                    .expect("Failed to notify error when editing task");
                            return;
                        }

                        let new_description: Option<String> =
                            if let (Some(pattern), Some(replace)) = (pattern, replace) {
                                if task.description.contains(&pattern) {
                                    displayer
                                        .notify(
                                            format!(
                                                "Replacing pattern '{}' with '{}'",
                                                pattern, replace
                                            )
                                            .as_ref(),
                                        )
                                        .expect("Failed when notifing edition of a task");

                                    Some(task.description.replace(&pattern, &replace))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                        if let Some(priority) = priority {
                            if !task.priority.eq(&priority) {
                                displayer
                                    .notify(
                                        format!(
                                            "Replacing task priority from '{}' to '{}'",
                                            task.priority, priority
                                        )
                                        .as_ref(),
                                    )
                                    .expect("Failed when notifing edition of a task");
                            }
                        }

                        manager.edit_task(
                            id.as_ref(),
                            (if new_description.is_some() {
                                new_description.unwrap()
                            } else {
                                task.description.clone()
                            })
                            .as_ref(),
                            &(if priority.is_some() {
                                priority.unwrap()
                            } else {
                                task.priority
                            }),
                        );
                    }
                }
                None => {
                    displayer.handle_edit_task(manager);
                }
            },
        }
    }

    fn is_task(id: &str, manager: &mut Manager, displayer: &mut dyn TraitCliDisplayer) -> bool {
        if manager.get_task(&id).is_none() {
            displayer
                .notify(&format!("Error: Task with ID {} not found", id))
                .expect("Failed to notify error for toggling task status");
            return false;
        } else {
            return true;
        }
    }
}
