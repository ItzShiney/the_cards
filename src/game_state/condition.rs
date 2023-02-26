use std::fmt::Display;

use itertools::Itertools;

use crate::{cs, default_formatted::DefaultFormatted};

pub enum Condition {
    IsBattle,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Condition::IsBattle => "битва".fmt(f),
        }
    }
}

impl Display for DefaultFormatted<&Vec<Condition>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = format!(" {} ", cs![And()]);
        self.0.iter().map(|condition| condition.to_string()).join(sep.as_str()).fmt(f)
    }
}
