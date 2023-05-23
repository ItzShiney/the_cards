use crate::chrs::CharacterType;
use crate::default_formatted::DefaultFormatted;
use crate::game_formatted::GameFormatted;
use crate::game_state::chr_id::CharacterID;
use crate::stats::Stats;
use std::fmt;
use std::fmt::Display;

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

impl Display for GameFormatted<'_, '_, '_, &CharacterInfo, CharacterID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.value.type_.name();
        let groups = self.value.type_.groups();
        let stats = &self.value.stats;
        let description = self.value.type_.description();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        writeln!(f, "\n{}", self.with_value(stats))?;

        if !description.slices.is_empty() {
            write!(f, "\n{}", description)?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
