use std::fmt::{self, Display, Formatter};

use crate::{acts::ActiveType, default_formatted::DefaultFormatted};

#[derive(Clone)]
pub struct ActiveInfo {
    pub type_: ActiveType,
}

impl ActiveInfo {
    pub fn new(type_: ActiveType) -> Self {
        Self { type_ }
    }
}

impl Display for ActiveInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = self.type_.name();
        let groups = self.type_.groups();
        let description = self.type_.description();

        writeln!(f, "──────────────────────────────────────")?;

        writeln!(f, "\x1b[1m{}\x1b[0m", name)?;
        if !groups.is_empty() {
            writeln!(f, "{}", DefaultFormatted(groups))?;
        }

        if let Some(description) = description {
            write!(f, "\n{}", description)?;
        }

        write!(f, "──────────────────────────────────────")
    }
}
