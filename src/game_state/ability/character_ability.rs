use crate::game_state::character_id::CharacterID;

use super::{character_trigger::CharacterTrigger, Ability};

pub type CharacterAbility = Ability<CharacterTrigger, CharacterID>;
