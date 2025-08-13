# To-Do List in Rust 🦀

![Rust](https://img.shields.io/badge/Rust-1.80+-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)
[![codecov](https://codecov.io/gh/francimadevilla/ToDo/graph/badge.svg?token=OO8IJN46YX)](https://codecov.io/gh/francimadevilla/ToDo)

Welcome to the **To-Do List in Rust** project, a console-based mini-project designed to learn and practice the fundamental features of the **Rust** programming language. This project is ideal for beginners who want to explore concepts such as *ownership*, *borrowing*, *structs*, *enums*, *pattern matching*, and more while building a practical and functional application.

## 🎯 Objective

The goal of this project is to create a console application that manages a to-do list. Users can add, list, complete, and delete tasks, as well as save them to a JSON file for persistence. This project is designed to:

- **Learn Rust**: Practice key language concepts in a hands-on way.
- **Build something useful**: Create a simple tool for task management.
- **Scalability**: Serve as a foundation for adding more features and continuing learning.

## 🚀 Installation and Execution

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (version 1.80 or higher) and Cargo installed.
- A text editor (recommended: [VS Code](https://code.visualstudio.com/) with the `rust-analyzer` extension).

### Setup Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/franCimadevilla/ToDo.git
   cd ToDo
   ```

2. Build and run the project:
   ```bash
   cargo build
   .\target\debug\ToDo.exe
   ```
   or
   ```bash
   cargo run
   ```

3. (Optional) Run tests and format code:
   ```bash
   cargo test
   cargo fmt
   ```

## 🖥️ Command Line Interface Commands

The `ToDo` application supports a Command Line Interface (CLI) for managing tasks directly from the terminal. The CLI is built using the `clap` crate and supports the following subcommands, each with specific arguments and options. The tasks are saved to `todo_list.json` for persistence.

### Usage
Run the program with a subcommand:
```bash
.\target\debug\ToDo.exe <SUBCOMMAND> [OPTIONS]
```
or
```bash
cargo run -- <SUBCOMMAND> [OPTIONS]
```

Use `--help` to see all available commands and options:
```bash
.\target\debug\ToDo.exe --help
```

### Available Subcommands

1. **`add`**
   - **Description**: Adds a new task to the to-do list.
   - **Arguments**:
     - `-d, --description <DESCRIPTION>`: The task description (required, free text).
     - `-p, --priority <PRIORITY>`: The task priority (`low`, `medium`, `high`). Case-insensitive (e.g., `Low` or `low`). Defaults to `low` if not specified or invalid.
   - **Output**: Displays "Task added successfully."
   - **Example**:
     ```bash
     .\target\debug\ToDo.exe add -d "Buy groceries" -p High
     ```
     Output: `Task added successfully.`
     ```bash
     cargo run -- add -d "Write report"  # Uses default priority (low)
     ```
     Output: `Task added successfully.`

2. **`list`**
   - **Description**: Lists tasks, optionally filtered by priority or completion status.
   - **Arguments**:
     - `-p, --priority <PRIORITY>`: Filter tasks by priority (`low`, `medium`, `high`). Case-insensitive. Optional.
     - `-c, --completed <true|false>`: Filter tasks by completion status (`true` for completed, `false` for pending). Optional.
   - **Output**:
     - If tasks are found, displays the number of tasks and their details in the format: `ID: X, Description: XXX, Priority: XXX, Completed: XXX`.
     - If no tasks match the filters, displays a message like "No tasks found" or "No tasks found with priority X and completed = Y."
   - **Examples**:
     ```bash
     .\target\debug\ToDo.exe list
     ```
     Output: 
     ```
     2 tasks found
     ID: 1, Description: Buy groceries, Priority: High, Completed: false
     ID: 2, Description: Write report, Priority: Low, Completed: false
     ```
     ```bash
     cargo run -- list -p Medium
     ```
     Output: `No tasks found with priority Medium`
     ```bash
     .\target\debug\ToDo.exe list -c true
     ```
     Output: `No tasks found with completed = true`

3. **`toggle-status`** (Aliases: `toggle`, `check`)
   - **Description**: Toggles the completion status of a task (completed to pending or vice versa).
   - **Arguments**:
     - `-i, --id <ID>`: The ID of the task to toggle (required, positive integer).
   - **Output**:
     - On success: "Task status toggled successfully."
     - On error (invalid ID): "Error: Task with ID X not found."
   - **Examples**:
     ```bash
     .\target\debug\ToDo.exe toggle-status -i 1
     ```
     Output: `Task status toggled successfully.`
     ```bash
     cargo run -- toggle -i 1
     ```
     Output: `Task status toggled successfully.`
     ```bash
     .\target\debug\ToDo.exe check -i 999
     ```
     Output: `Error: Task with ID 999 not found`

4. **`remove`**
   - **Description**: Deletes a task from the list.
   - **Arguments**:
     - `-i, --id <ID>`: The ID of the task to delete (required, positive integer).
   - **Output**:
     - On success: "Task removed successfully."
     - On error (invalid ID): "Error: Task with ID X not found."
   - **Example**:
     ```bash
     .\target\debug\ToDo.exe remove -i 1
     ```
     Output: `Task removed successfully.`
     ```bash
     cargo run -- remove -i 999
     ```
     Output: `Error: Task with ID 999 not found`

### Notes
- **Case Insensitivity**: Priority arguments (`low`, `medium`, `high`) are case-insensitive (e.g., `Low` and `low` are equivalent).
- **Task Persistence**: All tasks are saved to `todo_list.json` after each operation (`add`, `toggle-status`, `remove`) and loaded on program startup.
- **Error Handling**: Invalid inputs (e.g., non-numeric IDs, invalid priorities) are handled gracefully with appropriate error messages.
- **Interactive Mode**: Run the program without arguments (`.\target\debug\ToDo.exe`) to enter interactive mode, where you can select options from a menu.

## 🧰 Dependencies
The project uses the following crates (specified in `Cargo.toml`):
- `serde` and `serde_json`: For serialization/deserialization of tasks in JSON format.
- `tempfile`: For creating temporary files for testing purposes.
- `clap`: For parsing command-line arguments passed to the program.

## 📚 Resources
- [Official Rust Documentation](https://doc.rust-lang.org)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Community](https://users.rust-lang.org) for questions and support.
- [Clap - Rust](https://docs.rs/clap/latest/clap/)

## 🤝 Contributions
This is a learning project! If you have suggestions or improvements, feel free to open an *issue* or a *pull request* on GitHub.

## 📜 License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
