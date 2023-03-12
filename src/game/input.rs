mod _macro;
mod choose_card_args;
mod choose_card_args_p;
mod default_random;
mod default_random_bool;

use crate::game::state::act_id::ActiveID;
use crate::game::state::chr_id::CharacterID;
use crate::game::state::GameState;
use crate::game_input;
use crate::stats::Stat0;
pub use choose_card_args::*;
pub use choose_card_args_p::*;
pub use default_random::*;
pub use default_random_bool::*;

// TODO?
// #[game_input]
// pub trait GameInput
game_input! {
    fn random(&mut self, min: Stat0, max: Stat0) -> Stat0;
    fn random_bool(&mut self, true_prob: f64) -> bool;

    fn choose_chr_in_hand<'prompt_str, 'game_state>(&mut self, game_state: &'game_state GameState, args: ChooseCardArgsP<'prompt_str, 'game_state, '_, CharacterID>) -> Option<CharacterID>;
    fn choose_act_in_hand<'prompt_str, 'game_state>(&mut self, game_state: &'game_state GameState, args: ChooseCardArgsP<'prompt_str, 'game_state, '_, ActiveID>) -> Option<ActiveID>;
    fn choose_chr_on_field<'prompt_str, 'game_state>(&mut self, game_state: &'game_state GameState, args: ChooseCardArgsP<'prompt_str, 'game_state, '_, CharacterID>) -> Option<CharacterID>;
    fn choose_act_on_field<'prompt_str, 'game_state>(&mut self, game_state: &'game_state GameState, args: ChooseCardArgsP<'prompt_str, 'game_state, '_, ActiveID>) -> Option<ActiveID>;
}
