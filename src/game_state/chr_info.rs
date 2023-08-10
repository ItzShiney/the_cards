use {
    crate::{
        chr_uses::Signature,
        chrs::CharacterType,
        default_formatted::DefaultFormatted,
        game_state::chr_id::CharacterID,
        stats::Stats,
        GameFormatted,
    },
    std::{
        fmt,
        fmt::Display,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct CharacterInfo {
    pub type_: CharacterType,
    pub stats: Stats,
}

impl CharacterInfo {
    pub fn new(type_: CharacterType) -> Self {
        Self {
            type_,
            stats: type_.stats(),
        }
    }
}

impl Display for GameFormatted<'_, '_, '_, (CharacterID, Signature)> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (chr_id, signature) = self.value;
        let card = self.game.state.chr(chr_id);

        let name = card.type_.name();
        let groups = card.type_.groups();
        let stats = card.stats;
        let description = card.type_.description();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        writeln!(f, "\n{}", self.with_value((stats, chr_id, signature)))?;

        if !description.slices.is_empty() {
            write!(f, "\n{}", description)?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
