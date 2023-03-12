#![allow(clippy::uninlined_format_args)]
pub mod acts;
pub mod chrs;
pub mod console;
pub mod custom_string;
pub mod default_formatted;
pub mod effect;
pub mod game;
pub mod group;
pub mod stats;

use crate::game::state::act_info::ActiveInfo;
use crate::game::state::chr_info::CharacterInfo;
use acts::ActiveType;
use chrs::CharacterType;
use console::prompt_idxs;
use game::input::ChooseCardArgs;
use game::input::DefaultRandom;
use game::input::DefaultRandomBool;
use game::input::GameInputTuple;
use game::state::GameState;
use game::state::Player;
use game::Game;

pub fn print_chrs() {
    for chr_type in CharacterType::all() {
        println!("{}", CharacterInfo::new(chr_type));
    }
}

pub fn print_acts() {
    for act_type in ActiveType::all() {
        println!("{}", ActiveInfo::new(act_type));
    }
}

fn main() {
    let state = GameState::new(vec![
        Player { nickname: "Shiney".into() },
        Player { nickname: "maxvog".into() },
    ]);

    let input = Box::new(GameInputTuple {
        random: DefaultRandom,
        random_bool: DefaultRandomBool,
        choose_chr_in_hand: console::ChooseCharacterInHand,
        choose_act_in_hand: console::ChooseActiveInHand,
    });

    let mut game = Game::new(state, input);

    loop {
        let player_id = game.state().current_subturner_on_field().player_id;

        match prompt_idxs(
            "какое действие совершить?",
            false,
            ["выставить персонажа", "использовать активку"].into_iter(),
        ) {
            Some(0) => {
                let Some(chr_id) = game.choose_chr_in_hand_any(ChooseCardArgs {
                    prompt_str: &"какого персонажа выставить?",
                    is_cancellable: true,
                    player_id,
                }) else { continue };

                match game.place(chr_id) {
                    Err(_) => {
                        println!("{} не выставлен", cs![Character(game.state().chr(chr_id).type_)]);
                        continue;
                    }

                    Ok(_) => {
                        println!("{} выставлен", cs![Character(game.state().chr(chr_id).type_)]);
                    }
                }
            }

            Some(1) => {
                let Some(act_id) = game.choose_act_in_hand_any(ChooseCardArgs {
                    prompt_str: &"какую активку использовать?",
                    is_cancellable: true,
                    player_id,
                }) else { continue };

                todo!("{act_id:?}");
            }

            _ => unreachable!(),
        }
    }
}
