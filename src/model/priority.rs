use serde::{Serialize, Deserialize};
use clap::ValueEnum;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High
}

// Implement ValueEnum for Priority to make it case-insensitive
impl ValueEnum for Priority {
    fn value_variants<'a>() -> &'a [Self] {
        &[Priority::Low, Priority::Medium, Priority::High]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Priority::Low => Some(clap::builder::PossibleValue::new("low").alias("Low")),
            Priority::Medium => Some(clap::builder::PossibleValue::new("medium").alias("Medium")),
            Priority::High => Some(clap::builder::PossibleValue::new("high").alias("High")),
        }
    }
}