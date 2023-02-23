use std::fmt::{self, Display, Formatter};

use crate::{acts::ActiveType, default_formatted::DefaultFormatted, gendered::Gendered};

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
        let ru_gender = self.type_.ru_gender();

        write!(
            f,
            "──────────────────────────────────────\n\x1b[1m{}\x1b[0m\n{}\n{}──────────────────────────────────────",
            self.type_.name(),
            DefaultFormatted(self.type_.groups()),
            Gendered { ru_gender, value: self.type_.abilities() },
        )
    }
}
