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

#[derive(Subcommand, Clone, Debug)]
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
    pub fn evaluate_command(&self, command : CliCommand, manager: &mut Manager, mut displayer : Box<dyn Displayer>) {
        match command {
            CliCommand::Add { description, priority } => {
                manager.add_task(description.to_string(), priority);
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
                manager.remove_task(id.to_string());
                displayer.notify("Task removed successfully.")
                    .expect("Failed to notify removal of a task.");
            },
            CliCommand::ToggleStatus { id } => {
                manager.toggle_task_status(id.to_string());
                displayer.notify("Task status toggled successfully.")
                    .expect("Failed to notify toggling of task status.");
            }
        }
    }
}
