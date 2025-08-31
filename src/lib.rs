
pub mod model {
    pub mod priority;
    pub mod task;
    pub mod todo_list;
}

pub mod service {
    pub mod actions;
    pub mod manager;
}

pub mod ui {
    pub mod console_ui {
        pub mod console_displayer;
        pub mod menu_option;
        pub mod generic_console_displayer;
    }
    pub mod cli_argument_parser {
        pub mod cli_parser;
        pub mod cli_displayer;
    }
    pub mod displayer_trait;
}