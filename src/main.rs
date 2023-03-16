#![allow(clippy::uninlined_format_args)]
#![allow(clippy::nonminimal_bool)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces)]

pub mod acts;
mod card_uses;
pub mod chrs;
pub mod console;
pub mod custom_string;
pub mod default_formatted;
pub mod effect;
pub mod game;
pub mod group;
pub mod stats;

use crate::game::input::PromptArgs;
use crate::game::state::act_info::ActiveInfo;
use crate::game::state::chr_info::CharacterInfo;
use acts::ActiveType;
use chrs::CharacterType;
use console::prompt;
use game::input::DefaultRandom;
use game::input::DefaultRandomBool;
use game::input::GameInputTuple;
use game::state::act_id::ActiveID;
use game::state::chr_id::CharacterID;
use game::state::GameState;
use game::state::Player;
use game::Game;
use std::iter::repeat;

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

    CheckField,
    CheckOwnCharacter,
    CheckEnemyCharacter,

    CardsList,

    CharactersList,
    CharacterOptions {
        chr_id: CharacterID,
    },
    CheckCharacter {
        chr_id: CharacterID,
    },
    PlaceCharacter {
        chr_id: CharacterID,
    },

    ActivesList,
    ActiveOptions {
        act_id: ActiveID,
    },
    CheckActive {
        act_id: ActiveID,
    },
    UseActive {
        act_id: ActiveID,
    },
    UseActiveOnField {
        act_id: ActiveID,
    },
    UseActiveOnOwnCharacter {
        act_id: ActiveID,
    },
    UseActiveOnEnemyCharacter {
        act_id: ActiveID,
    },

    EndSubturn,
}

impl InputState {
    pub fn previous(self) -> Self {
        use InputState::*;

        match self {
            ChooseAction => self,

            CheckField => ChooseAction,
            CheckOwnCharacter => CheckField,
            CheckEnemyCharacter => CheckField,

            CardsList => ChooseAction,

            CharactersList => CardsList,
            CharacterOptions { chr_id: _ } => CharactersList,
            CheckCharacter { chr_id } => CharacterOptions { chr_id },
            PlaceCharacter { chr_id } => CharacterOptions { chr_id },

            ActivesList => CardsList,
            ActiveOptions { act_id: _ } => ActivesList,
            CheckActive { act_id } => ActiveOptions { act_id },
            UseActive { act_id } => ActiveOptions { act_id },
            UseActiveOnField { act_id } => UseActive { act_id },
            UseActiveOnOwnCharacter { act_id } => UseActive { act_id },
            UseActiveOnEnemyCharacter { act_id } => UseActive { act_id },

            EndSubturn => ChooseAction,
        }
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

        let mut state = InputState::default();
        loop {
            let player_id = game.state().current_subturner_on_field().player_id;

            state = match state {
                ChooseAction => {
                    let player_nickname = game.state().players_map[&player_id].nickname.clone();
                    let can_end_subturn = game.state().attacker.player_id != player_id
                        || game.state().attacker.chr_id.is_some();

                    match prompt(
                        PromptArgs {
                            str: cs![Name(cs![player_nickname])],
                            is_cancellable: false,
                            autochoose_single_option: false,
                        },
                        [
                            (true, cs!["поле"]),
                            (true, cs!["рука"]),
                            (can_end_subturn, cs!["закончить подход"]),
                        ]
                        .into_iter(),
                    ) {
                        Some(0) => CheckField,
                        Some(1) => CardsList,
                        Some(2) => EndSubturn,
                        _ => unreachable!(),
                    }
                }

                CheckField => {
                    let is_own_chr_placed = game.state().try_own_chr_id(player_id).is_some();
                    let is_enemy_chr_placed = game.state().try_enemy_chr_id(player_id).is_some();

                    match prompt(
                        PromptArgs {
                            str: cs!["поле"],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        [
                            (is_own_chr_placed, cs!["свой персонаж"]),
                            (is_enemy_chr_placed, cs!["персонаж противника"]),
                        ]
                        .into_iter(),
                    ) {
                        None => state.previous(),
                        Some(0) => CheckOwnCharacter,
                        Some(1) => CheckEnemyCharacter,
                        _ => unreachable!(),
                    }
                }

                CheckOwnCharacter => {
                    let chr_id = game.state().own_chr_id(player_id);
                    println!("{}", game.state().chr(chr_id));

                    state.previous()
                }

                CheckEnemyCharacter => {
                    let chr_id = game.state().enemy_chr_id(player_id);
                    println!("{}", game.state().chr(chr_id));

                    state.previous()
                }

                CardsList => {
                    let has_chrs = !game.state().chrs.hand(player_id).is_empty();
                    let has_acts = !game.state().acts.hand(player_id).is_empty();

                    match prompt(
                        PromptArgs {
                            str: cs!["рука: тип карт"],
                            is_cancellable: true,
                            autochoose_single_option: true,
                        },
                        [(has_chrs, cs!["персонажи"]), (has_acts, cs!["активки"])].into_iter(),
                    ) {
                        None => state.previous(),
                        Some(0) => CharactersList,
                        Some(1) => ActivesList,
                        _ => unreachable!(),
                    }
                }

                CharactersList => {
                    let chr_ids = game.state().chrs.hand(player_id);
                    let chr_names = chr_ids
                        .iter()
                        .copied()
                        .map(|chr_id| cs![Character(game.state().chr(chr_id).type_)]);
                    let options = repeat(true).zip(chr_names);

                    match prompt(
                        PromptArgs {
                            str: cs!["персонажи"],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        options,
                    ) {
                        None => state.previous(),

                        Some(chr_id_idx) => {
                            let chr_id = chr_ids[chr_id_idx];

                            CharacterOptions { chr_id }
                        }
                    }
                }

                CharacterOptions { chr_id } => {
                    match prompt(
                        PromptArgs {
                            str: cs![Character(game.state().chr(chr_id).type_)],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        [
                            (true, cs!["просмотреть"]),
                            (game.state().is_placeable(chr_id), cs!["выставить"]),
                        ]
                        .into_iter(),
                    ) {
                        None => state.previous(),
                        Some(0) => CheckCharacter { chr_id },
                        Some(1) => PlaceCharacter { chr_id },
                        _ => unreachable!(),
                    }
                }

                CheckCharacter { chr_id } => {
                    println!("{}", game.state().chr(chr_id));

                    state.previous()
                }

                PlaceCharacter { chr_id } => {
                    game.place(chr_id).unwrap();

                    println!(
                        "персонаж {} выставлен",
                        cs![Character(game.state().chr(chr_id).type_)]
                    );

                    InputState::default()
                }

                ActivesList => {
                    let act_ids = game.state().acts.hand(player_id);
                    let act_names = act_ids
                        .iter()
                        .copied()
                        .map(|act_id| cs![Active(game.state().act(act_id).type_)]);
                    let options = repeat(true).zip(act_names);

                    match prompt(
                        PromptArgs {
                            str: cs!["активки"],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        options,
                    ) {
                        None => state.previous(),

                        Some(act_id_idx) => {
                            let act_id = act_ids[act_id_idx];

                            ActiveOptions { act_id }
                        }
                    }
                }

                ActiveOptions { act_id } => {
                    match prompt(
                        PromptArgs {
                            str: cs![Active(game.state().act(act_id).type_)],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        [
                            (true, cs!["просмотреть"]),
                            (game.state().is_usable_in_any_way(act_id), cs!["использовать"]),
                        ]
                        .into_iter(),
                    ) {
                        None => state.previous(),
                        Some(0) => CheckActive { act_id },
                        Some(1) => UseActive { act_id },
                        _ => unreachable!(),
                    }
                }

                CheckActive { act_id } => {
                    println!("{}", game.state().act(act_id));

                    state.previous()
                }

                UseActive { act_id } => {
                    let is_usable_on_field = game.state().is_usable_on_field(act_id);

                    let is_usable_on_own_chr = game.state().try_own_chr_id(player_id).is_some()
                        && game.state().is_usable_on_own_chr(act_id);

                    let is_usable_on_enemy_chr = game.state().try_enemy_chr_id(player_id).is_some()
                        && game.state().is_usable_on_enemy_chr(act_id);

                    match prompt(
                        PromptArgs {
                            str: cs!["использовать активку"],
                            is_cancellable: true,
                            autochoose_single_option: false,
                        },
                        [
                            (is_usable_on_field, cs!["на поле"]),
                            (is_usable_on_own_chr, cs!["на своего персонажа"]),
                            (is_usable_on_enemy_chr, cs!["на противника"]),
                        ]
                        .into_iter(),
                    ) {
                        None => state.previous(),
                        Some(0) => UseActiveOnField { act_id },
                        Some(1) => UseActiveOnOwnCharacter { act_id },
                        Some(2) => UseActiveOnEnemyCharacter { act_id },
                        _ => unreachable!(),
                    }
                }

                UseActiveOnField { act_id } => {
                    game.use_on_field(act_id).unwrap();

                    println!(
                        "активка {} использована на поле",
                        cs![Active(game.state().act(act_id).type_)]
                    );

                    InputState::default()
                }

                UseActiveOnOwnCharacter { act_id } => {
                    let target_id = game.state().own_chr_id(player_id);
                    game.use_on_chr(act_id, target_id).unwrap();

                    println!(
                        "активка {} использована на персонажа {}",
                        cs![Active(game.state().act(act_id).type_)],
                        cs![Character(game.state().chr(target_id).type_)],
                    );

                    InputState::default()
                }

                UseActiveOnEnemyCharacter { act_id } => {
                    let target_id = game.state().enemy_chr_id(player_id);
                    game.use_on_chr(act_id, target_id).unwrap();

                    println!(
                        "активка {} использована на персонажа {}",
                        cs![Active(game.state().act(act_id).type_)],
                        cs![Character(game.state().chr(target_id).type_)],
                    );

                    InputState::default()
                }

                EndSubturn => {
                    game.end_subturn();

                    println!("подход завершён");

                    InputState::default()
                }
            };

            println!();
        }
    }
}
