use std::fmt::{self, Display, Formatter};

use crate::{chrs::CharacterType, default_formatted::DefaultFormatted, stats::Stats};

#[derive(Clone)]
pub struct CharacterInfo {
    pub type_: CharacterType,
    pub stats: Stats,
}

impl CharacterInfo {
    pub fn new(type_: CharacterType) -> Self {
        Self { type_, stats: type_.stats() }
    }
}

impl Display for CharacterInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = self.type_.name();
        let groups = self.type_.groups();
        let stats = &self.stats;
        let description = self.type_.description();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        writeln!(f, "\n{}", stats)?;

        if let Some(description) = description {
            write!(f, "\n{}", description)?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
