use crate::game::state::player_id::PlayerID;
use std::fmt::Display;

pub struct ChooseCardArgs<'prompt_str> {
    pub prompt_str: &'prompt_str dyn Display,
    pub is_cancellable: bool,
    pub player_id: PlayerID,
}
