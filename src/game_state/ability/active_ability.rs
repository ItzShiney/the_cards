use std::fmt::Display;

use crate::{
    game_state::{ActiveID, CharacterID},
    gendered::{Gendered, RuGender},
    trigger_trait::TriggerTrait,
};

use super::Ability;

#[derive(Clone, Copy)]
pub enum UseTarget {
    Field,
    Character,
    OwnCharacter,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveTrigger {
    UsedOnField,
    UsedOnCharacter,
    UsedOnOwnCharacter,
}

impl Display for Gendered<&ActiveTrigger> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let использовано = match self.ru_gender {
            RuGender::Masculine => "использован",
            RuGender::Feminine => "использована",
            RuGender::Neuter => "использовано",
            RuGender::Plural => "использованы",
        };

        match self.value {
            ActiveTrigger::UsedOnField => write!(f, "{использовано}"),
            ActiveTrigger::UsedOnCharacter => write!(f, "{использовано} на персонажа"),
            ActiveTrigger::UsedOnOwnCharacter => write!(f, "{использовано} на своего персонажа"),
        }
    }
}

impl TriggerTrait for ActiveTrigger {
    type Went = WentActiveTrigger;
}

pub enum WentActiveTrigger {
    UsedOnField,
    UsedOnCharacter(CharacterID),
}

pub type ActiveAbility = Ability<ActiveTrigger, ActiveID>;
