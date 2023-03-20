use crate::card_uses::Stat0;
use crate::game_input::DefaultRandom;
use crate::game_input::GameInput;

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

    choose_card_on_field!(Character, chr, |state| {
        let mut res = vec![];

        if let Some(chr_id) = state.attacker.chr_id {
            res.push(chr_id);
        }

        if let Some(chr_id) = state.defender.chr_id {
            res.push(chr_id);
        }

        res
    });

    choose_card_on_field!(Active, act, |state| {
        state
            .attacker
            .used_act_ids
            .iter()
            .copied()
            .chain(state.defender.used_act_ids.iter().copied())
            .collect()
    });
}
