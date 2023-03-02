pub mod act_trigger;
pub mod chr_trigger;

use std::fmt::Display;

use crate::custom_string::CustomString;

pub struct AbilityDescription {
    pub name: Option<CustomString>,
    pub description: CustomString,
}

impl Display for AbilityDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        }
        write!(f, "{}", self.description)
    }
}
