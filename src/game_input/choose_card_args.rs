use crate::custom_string::CustomString;
use crate::game::Game;
use crate::game_state::player_id::PlayerID;

pub struct PromptArgs {
    pub str: CustomString,
    pub is_cancellable: bool,
    pub autochoose_single_option: bool,
}

pub struct ChooseCardArgs {
    pub prompt: PromptArgs,
    pub player_id: PlayerID,
}

pub struct ChooseCardArgsP<'p, ID> {
    pub prompt: PromptArgs,
    pub player_id: PlayerID,
    pub p: &'p dyn Fn(&mut Game, ID) -> bool,
}

impl<'p, ID> ChooseCardArgsP<'p, ID> {
    pub fn new(
        ChooseCardArgs { prompt, player_id }: ChooseCardArgs,
        p: &'p dyn Fn(&mut Game, ID) -> bool,
    ) -> Self {
        Self { prompt, player_id, p }
    }
}
