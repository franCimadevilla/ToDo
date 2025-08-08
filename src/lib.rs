
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
        pub mod menu_options;
    }
    pub mod displayer_trait;
}