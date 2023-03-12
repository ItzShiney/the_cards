use super::ChooseCardArgs;
use crate::game::state::player_id::PlayerID;
use crate::game::state::GameState;
use std::fmt::Display;

pub struct ChooseCardArgsP<'prompt_str, 'game_state, 'p, ID> {
    pub prompt: &'prompt_str dyn Display,
    pub is_cancellable: bool,
    pub player_id: PlayerID,
    pub p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
}

impl<'prompt_str, 'game_state, 'p, ID> ChooseCardArgsP<'prompt_str, 'game_state, 'p, ID> {
    pub fn new(
        ChooseCardArgs { prompt, player_id, is_cancellable }: ChooseCardArgs<'prompt_str>,
        p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
    ) -> Self {
        Self { prompt, player_id, is_cancellable, p }
    }
}
