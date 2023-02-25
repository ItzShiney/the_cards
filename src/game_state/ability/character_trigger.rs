use std::fmt::Display;

use crate::{
    gendered::{Gendered, RuGender},
    trigger_trait::TriggerTrait,
};

pub enum CharacterTrigger {
    Placed,
}

impl TriggerTrait for CharacterTrigger {
    type Went = WentCharacterTrigger;
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

pub enum WentCharacterTrigger {
    Placed,
}
