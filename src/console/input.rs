use crate::{
    card_uses::Stat0,
    game_input::{
        DefaultRandom,
        GameInput,
    },
};

#[macro_use]
mod choose_card_in_hand;
#[macro_use]
mod choose_card_on_field;

pub struct ConsoleInput;

impl GameInput for ConsoleInput {
    fn random(&mut self, min: Stat0, max: Stat0) -> Stat0 {
        DefaultRandom.random(min, max)
    }

    fn random_bool(&mut self, true_prob: f64) -> bool {
        DefaultRandom.random_bool(true_prob)
    }

    choose_card_in_hand!(Character, chr);
    choose_card_in_hand!(Active, act);

    choose_card_on_field!(Character, chr, |state| state.all_chrs_on_field().collect());

    choose_card_on_field!(Active, act, |state| state.all_acts_on_field().collect());
}
