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
        let ru_gender = self.type_.ru_gender();

        write!(
            f,
            "──────────────────────────────────────\n\x1b[1m{}\x1b[0m\n{}\n{}\n{}──────────────────────────────────────",
            self.type_.name(),
            DefaultFormatted(self.type_.groups()),
            self.stats,
            Gendered { ru_gender, value: self.type_.abilities() },
        )
    }
}
