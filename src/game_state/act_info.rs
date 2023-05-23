use crate::acts::ActiveType;
use crate::card_uses::ActiveID;
use crate::default_formatted::DefaultFormatted;
use crate::game_formatted::GameFormatted;
use std::fmt;
use std::fmt::Display;

#[derive(Clone)]
pub struct ActiveInfo {
    pub type_: ActiveType,
}

impl ActiveInfo {
    pub fn new(type_: ActiveType) -> Self {
        Self { type_ }
    }
}

impl Display for GameFormatted<'_, '_, '_, &ActiveInfo, ActiveID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.value.type_.name();
        let groups = self.value.type_.groups();
        let description = self.value.type_.description();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        if !description.slices.is_empty() {
            write!(f, "\n{}", description)?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
