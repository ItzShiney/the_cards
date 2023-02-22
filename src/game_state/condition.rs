use std::fmt::Display;

use crate::gendered::Gendered;

pub enum Condition {
    IsBattle,
}

impl Display for Gendered<&Condition> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self.value {
            Condition::IsBattle => "битва".fmt(f),
        }
    }
}
