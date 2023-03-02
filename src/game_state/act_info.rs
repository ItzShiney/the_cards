use std::fmt::{self, Display, Formatter};

use crate::{acts::ActiveType, default_formatted::DefaultFormatted};

#[derive(Clone)]
pub struct ActiveInfo {
    pub type_: ActiveType,
}

impl From<ActiveType> for ActiveInfo {
    fn from(type_: ActiveType) -> Self {
        Self { type_ }
    }
}

impl Display for ActiveInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = self.type_.name();
        let groups = self.type_.groups();
        let epitaph = self.type_.epitaph();
        let abilities = self.type_.abilities();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        if let Some(epitaph) = epitaph {
            writeln!(f, "\n\x1b[3m{}\x1b[0m", epitaph)?;
        }

        writeln!(f, "{}", abilities)?;

        write!(f, "──────────────────────────────────────")
    }
}
