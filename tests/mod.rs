#[cfg(test)]
mod ui {
    mod console_ui {
        #[path = "generic_console_displayer.rs"]
        mod generic_console_displayer;
    }
    mod cli_argument_parser {
        #[path = "cli_parser.rs"]
        mod cli_parser;
    }
}
mod model {
        #[path = "todo_list.rs"]
        mod todo_list;
}
