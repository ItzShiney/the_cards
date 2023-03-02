use std::fmt::Display;

use crate::gendered::{Gendered, RuGender};

pub enum CharacterTrigger {
    Placed,
}

impl Display for Gendered<&CharacterTrigger> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            CharacterTrigger::Placed => {
                let выставлен = match self.ru_gender {
                    RuGender::Masculine => "выставлен",
                    RuGender::Feminine => "выставлена",
                    RuGender::Neuter => "выставлено",
                    RuGender::Plural => "выставлены",
                };

                write!(f, "{выставлен}")
            }
        }
    }
}
