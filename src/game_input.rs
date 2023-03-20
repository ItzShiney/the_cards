mod _macro;
mod choose_card_args;
mod default_random;
mod default_random_bool;

use crate::game_input;
use crate::game_state::act_id::ActiveID;
use crate::game_state::chr_id::CharacterID;
use crate::game_state::GameState;
use crate::stats::Stat0;
pub use choose_card_args::*;
pub use default_random::*;
pub use default_random_bool::*;

// TODO?
// #[game_input]
// pub trait GameInput
game_input! {
    fn random(&mut self, min: Stat0, max: Stat0) -> Stat0;
    fn random_bool(&mut self, true_prob: f64) -> bool;

    fn choose_chr_in_hand(&mut self, game_state: &mut GameState, args: ChooseCardArgsP<'_, CharacterID>) -> Option<CharacterID>;
    fn choose_act_in_hand(&mut self, game_state: &mut GameState, args: ChooseCardArgsP<'_, ActiveID>) -> Option<ActiveID>;
    fn choose_chr_on_field(&mut self, game_state: &mut GameState, args: ChooseCardArgsP<'_, CharacterID>) -> Option<CharacterID>;
    fn choose_act_on_field(&mut self, game_state: &mut GameState, args: ChooseCardArgsP<'_, ActiveID>) -> Option<ActiveID>;
}
