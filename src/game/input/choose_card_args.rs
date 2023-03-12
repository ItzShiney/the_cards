use crate::custom_string::CustomString;
use crate::game::state::player_id::PlayerID;
use crate::game::state::GameState;

pub struct ChooseCardArgs<'prompt> {
    pub prompt: &'prompt CustomString,
    pub is_cancellable: bool,
    pub player_id: PlayerID,
}

pub struct ChooseCardArgsP<'prompt, 'game_state, 'p, ID> {
    pub prompt: &'prompt CustomString,
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
