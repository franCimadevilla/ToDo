use serde::{Serialize, Deserialize};
use clap::ValueEnum;
use std::{fmt::{Display, Formatter, Result}};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
    pub fn str_to_priority(text : &str) -> core::result::Result<Self, String> {
        match text.to_lowercase().as_str() {
            "1"|"high" => Ok(Priority::High),
            "2"|"medium" => Ok(Priority::Medium),
            "3"|"low" => Ok(Priority::Low),
            other => Err(format!("Invalid prority value: {}", other))
        } 
    }
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

impl Display for Priority {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        let text = match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High"
        };
        write!(f, "{}", text)
    }
}