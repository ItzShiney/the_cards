#![allow(clippy::uninlined_format_args)]
#![allow(clippy::nonminimal_bool)]

pub mod acts;
pub mod chrs;
pub mod console;
pub mod custom_string;
pub mod default_formatted;
pub mod effect;
pub mod game;
pub mod group;
pub mod stats;

use crate::game::input::ChooseCardArgs;
use crate::game::state::act_id::ActiveID;
use crate::game::state::act_info::ActiveInfo;
use crate::game::state::chr_info::CharacterInfo;
use acts::ActiveType;
use chrs::CharacterType;
use console::prompt;
use crossterm::style::Stylize;
use game::input::ChooseCardArgsP;
use game::input::DefaultRandom;
use game::input::DefaultRandomBool;
use game::input::GameInputTuple;
use game::state::chr_id::CharacterID;
use game::state::GameState;
use game::state::Player;
use game::Game;
use std::mem::take;

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

#[derive(Default)]
pub enum InputState {
    #[default]
    ChooseAction,

    ChooseCharacter,
    PlaceCharacter {
        chr_id: CharacterID,
    },

    ChooseActive,
    UseHow {
        act_id: ActiveID,
    },
    UseOnField {
        act_id: ActiveID,
    },
    UseOnWho {
        act_id: ActiveID,
    },
    UseOnCharacter {
        act_id: ActiveID,
        target_id: CharacterID,
    },

    EndSubturn,
}

impl InputState {
    pub fn previous(self) -> Self {
        use InputState::*;

        match self {
            ChooseAction => self,

            ChooseCharacter => ChooseAction,
            PlaceCharacter { chr_id: _ } => ChooseCharacter,

            ChooseActive => ChooseAction,
            UseHow { act_id: _ } => ChooseActive,
            UseOnField { act_id } => UseHow { act_id },
            UseOnWho { act_id } => UseHow { act_id },
            UseOnCharacter { act_id, target_id: _ } => UseOnWho { act_id },

            EndSubturn => ChooseAction,
        }
    }

    pub fn revert(&mut self) {
        *self = take(self).previous();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
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
        choose_chr_on_field: console::ChooseCharacterOnField,
        choose_act_on_field: console::ChooseActiveOnField,
    });

    let mut game = Game::new(state, input);

    {
        use InputState::*;

        let mut state = ChooseAction;
        loop {
            let player_id = game.state().current_subturner_on_field().player_id;

            match state {
                ChooseAction => {
                    match prompt(
                        game.state().players_map[&player_id].nickname.as_str().bold(),
                        false,
                        [
                            (true, "выставить персонажа"),
                            (true, "использовать активку"),
                            (true, "закончить подход"),
                        ]
                        .into_iter(),
                    ) {
                        None => state.revert(),
                        Some(0) => state = ChooseCharacter,
                        Some(1) => state = ChooseActive,
                        Some(2) => state = EndSubturn,
                        _ => unreachable!(),
                    }
                }

                ChooseCharacter => {
                    match game.choose_chr_in_hand(ChooseCardArgsP {
                        prompt: &cs!["какого персонажа выставить?"],
                        is_cancellable: true,
                        player_id,
                        p: &GameState::is_placeable,
                    }) {
                        None => state.revert(),
                        Some(chr_id) => state = PlaceCharacter { chr_id },
                    };
                }

                PlaceCharacter { chr_id } => {
                    game.place(chr_id).unwrap();
                    println!(
                        "персонаж {} выставлен",
                        cs![Character(game.state().chr(chr_id).type_)]
                    );

                    state.reset();
                }

                ChooseActive => {
                    match game.choose_act_in_hand(ChooseCardArgsP {
                        prompt: &cs!["какую активку использовать?"],
                        is_cancellable: true,
                        player_id,
                        p: &GameState::is_usable_in_any_way,
                    }) {
                        None => state.revert(),
                        Some(act_id) => state = UseHow { act_id },
                    }
                }

                UseHow { act_id } => {
                    let act_abilities = game.state().act(act_id).type_.abilities();

                    match prompt(
                        "как использовать?",
                        true,
                        [
                            (act_abilities.use_on_field.is_some(), "на поле"),
                            (act_abilities.use_on_chr.is_some(), "на персонажа"),
                        ]
                        .into_iter(),
                    ) {
                        None => state.revert(),
                        Some(0) => state = UseOnField { act_id },
                        Some(1) => state = UseOnWho { act_id },
                        _ => unreachable!(),
                    }
                }

                UseOnField { act_id } => {
                    game.use_on_field(act_id).unwrap();

                    state.reset();
                }

                UseOnWho { act_id } => {
                    match game.choose_chr_on_field_any(ChooseCardArgs {
                        prompt: &cs![
                            Active(game.state().act(act_id).type_),
                            ": на кого использовать?"
                        ],
                        is_cancellable: true,
                        player_id,
                    }) {
                        None => state.revert(),
                        Some(target_id) => state = UseOnCharacter { act_id, target_id },
                    }
                }

                UseOnCharacter { act_id, target_id } => {
                    game.use_on_chr(act_id, target_id).unwrap();

                    println!(
                        "активка {} использована на {}",
                        cs![Active(game.state().act(act_id).type_)],
                        cs![Character(game.state().chr(target_id).type_)],
                    );

                    state.reset();
                }

                EndSubturn => {
                    game.end_subturn();

                    println!(
                        "подход {} закончен",
                        game.state().players_map[&player_id].nickname.as_str().bold()
                    );

                    state.reset();
                }
            }
            println!();
        }
    }
}
