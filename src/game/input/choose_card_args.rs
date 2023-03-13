use crate::custom_string::CustomString;
use crate::game::state::player_id::PlayerID;
use crate::game::state::GameState;

pub struct PromptArgs {
    pub str: CustomString,
    pub is_cancellable: bool,
    pub autochoose_single_option: bool,
}

pub struct ChooseCardArgs {
    pub prompt: PromptArgs,
    pub player_id: PlayerID,
}

pub struct ChooseCardArgsP<'game_state, 'p, ID> {
    pub prompt: PromptArgs,
    pub player_id: PlayerID,
    pub p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
}

impl<'game_state, 'p, ID> ChooseCardArgsP<'game_state, 'p, ID> {
    pub fn new(
        ChooseCardArgs { prompt, player_id }: ChooseCardArgs,
        p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
    ) -> Self {
        Self { prompt, player_id, p }
    }
}
