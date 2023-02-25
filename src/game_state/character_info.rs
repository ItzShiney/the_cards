use std::fmt::{self, Display, Formatter};

use crate::{
    chrs::CharacterType, default_formatted::DefaultFormatted, gendered::Gendered, stats::Stats,
};

#[derive(Clone)]
pub struct CharacterInfo {
    pub type_: CharacterType,
    pub stats: Stats,
}

impl From<CharacterType> for CharacterInfo {
    fn from(type_: CharacterType) -> Self {
        Self { type_, stats: type_.stats() }
    }
}

impl Display for CharacterInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = self.type_.name();
        let groups = self.type_.groups();
        let stats = &self.stats;
        let epitaph = self.type_.epitaph();
        let ru_gender = self.type_.ru_gender();
        let abilities = self.type_.abilities();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        writeln!(f, "\n{}", stats)?;

        if let Some(epitaph) = epitaph {
            writeln!(f, "\n\x1b[3m{}\x1b[0m", epitaph)?;
        }

        if !abilities.is_empty() {
            writeln!(f, "\n{}", Gendered { ru_gender, value: abilities })?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
