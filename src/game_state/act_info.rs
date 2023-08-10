use {
    crate::{
        acts::ActiveType,
        card_uses::ActiveID,
        default_formatted::DefaultFormatted,
        GameFormatted,
    },
    std::{
        fmt,
        fmt::Display,
    },
};

#[derive(Clone)]
pub struct ActiveInfo {
    pub type_: ActiveType,
}

impl ActiveInfo {
    pub fn new(type_: ActiveType) -> Self {
        Self { type_ }
    }
}

impl Display for GameFormatted<'_, '_, '_, ActiveID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let act_id = self.value;
        let card = self.game.state.act(act_id);

        let name = card.type_.name();
        let groups = card.type_.groups();
        let description = card.type_.description();

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
