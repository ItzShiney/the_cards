use super::ChooseCardArgs;
use crate::game::state::player_id::PlayerID;
use crate::game::state::GameState;

pub struct ChooseCardArgsP<'game_state, 'p, ID> {
    pub is_cancellable: bool,
    pub player_id: PlayerID,
    pub p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
}

impl<'p, 'game_state, ID> ChooseCardArgsP<'p, 'game_state, ID> {
    pub fn new(
        ChooseCardArgs { player_id, is_cancellable }: ChooseCardArgs,
        p: &'p dyn Fn(&'game_state GameState, ID) -> bool,
    ) -> Self {
        Self { player_id, is_cancellable, p }
    }
}
