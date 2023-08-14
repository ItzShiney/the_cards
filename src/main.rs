#![allow(clippy::uninlined_format_args)]
#![allow(clippy::nonminimal_bool)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces
)]

mod act_uses;
pub mod acts;
mod card_uses;
mod chr_uses;
pub mod chrs;
pub mod console;
pub mod custom_string;
pub mod default_formatted;
pub mod game;
mod game_formatted;
pub mod game_input;
pub mod game_state;
pub mod group;
pub mod stats;

pub use game_formatted::*;
use {
    crate::{
        act_uses::UseWay,
        chr_uses::Sign,
        console::ConsoleInput,
        game::*,
    },
    console::prompt,
    game_input::PromptArgs,
    game_state::{
        act_id::ActiveID,
        chr_id::CharacterID,
        event::Event,
        GameState,
        Player,
    },
    std::iter::repeat,
};

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

pub fn main() {
    let mut state = GameState::new(vec![
        Player {
            nickname: "Shiney".into(),
        },
        Player {
            nickname: "maxvog".into(),
        },
    ]);

    let mut input = ConsoleInput;

    let mut game = Game {
        state: &mut state,
        input: &mut input,
    };

    // print_cards(&mut game);
    ui(&mut game);
}

#[allow(unused)]
fn print_cards(game: &mut Game) {
    todo!();
}

#[allow(unused)]
fn ui(game: &mut Game) {
    use InputState::*;

    let mut input_state = InputState::default();
    loop {
        let subturner = game.state.turn_info.subturner;
        let other_subturner = subturner.other();
        let player_id = game.state.turn_info.id_by_subturner(subturner);
        let enemy_player_id = game.state.turn_info.id_by_subturner(other_subturner);

        input_state = match input_state {
            ChooseAction => {
                let player_nickname = game.state.players_map[&player_id].nickname.clone();
                let can_end_subturn = game.can_end_subturn(player_id);

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
                let is_own_chr_placed = game.state.chrs_on_field(player_id).next().is_some();
                let is_enemy_chr_placed = game
                    .state
                    .chrs_on_field(game.state.turn_info.id_by_subturner(other_subturner))
                    .next()
                    .is_some();

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
                    None => input_state.previous(),
                    Some(0) => CheckOwnCharacter,
                    Some(1) => CheckEnemyCharacter,
                    _ => unreachable!(),
                }
            }

            CheckOwnCharacter => {
                let chr_id = game.state.chrs_on_field(player_id).next().unwrap();
                println!(
                    "{}",
                    GameFormatted {
                        game,
                        value: (chr_id, player_id.into())
                    }
                );

                input_state.previous()
            }

            CheckEnemyCharacter => {
                let chr_id = game.state.chrs_on_field(enemy_player_id).next().unwrap();
                println!(
                    "{}",
                    GameFormatted {
                        game,
                        value: (chr_id, player_id.into())
                    }
                );

                input_state.previous()
            }

            CardsList => {
                let has_chrs = !game.state.chrs.hand(player_id).is_empty();
                let has_acts = !game.state.acts.hand(player_id).is_empty();

                match prompt(
                    PromptArgs {
                        str: cs!["рука: тип карт"],
                        is_cancellable: true,
                        autochoose_single_option: true,
                    },
                    [(has_chrs, cs!["персонажи"]), (has_acts, cs!["активки"])].into_iter(),
                ) {
                    None => input_state.previous(),
                    Some(0) => CharactersList,
                    Some(1) => ActivesList,
                    _ => unreachable!(),
                }
            }

            CharactersList => {
                let chr_ids = game.state.chrs.hand(player_id);
                let chr_names = chr_ids
                    .iter()
                    .copied()
                    .map(|chr_id| cs![Character(game.state.chr(chr_id).type_)]);
                let options = repeat(true).zip(chr_names);

                match prompt(
                    PromptArgs {
                        str: cs!["персонажи"],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    options,
                ) {
                    None => input_state.previous(),

                    Some(chr_id_idx) => {
                        let chr_id = chr_ids.iter().copied().nth(chr_id_idx).unwrap();

                        CharacterOptions { chr_id }
                    }
                }
            }

            CharacterOptions { chr_id } => {
                match prompt(
                    PromptArgs {
                        str: cs![Character(game.state.chr(chr_id).type_)],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    [
                        (true, cs!["просмотреть"]),
                        (
                            game.can(Event::Place { chr_id }.sign(player_id)),
                            cs!["выставить"],
                        ),
                    ]
                    .into_iter(),
                ) {
                    None => input_state.previous(),
                    Some(0) => CheckCharacter { chr_id },
                    Some(1) => PlaceCharacter { chr_id },
                    _ => unreachable!(),
                }
            }

            CheckCharacter { chr_id } => {
                println!(
                    "{}",
                    GameFormatted {
                        game,
                        value: (chr_id, player_id.into())
                    }
                );

                input_state.previous()
            }

            PlaceCharacter { chr_id } => {
                Event::Place { chr_id }.sign(player_id).try_(game).unwrap();

                println!(
                    "персонаж {} выставлен",
                    cs![Character(game.state.chr(chr_id).type_)]
                );

                InputState::default()
            }

            ActivesList => {
                let act_ids = game.state.acts.hand(player_id);
                let act_names = act_ids
                    .iter()
                    .copied()
                    .map(|act_id| cs![Active(game.state.act(act_id).type_)]);
                let options = repeat(true).zip(act_names);

                match prompt(
                    PromptArgs {
                        str: cs!["активки"],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    options,
                ) {
                    None => input_state.previous(),

                    Some(act_id_idx) => {
                        let act_id = act_ids.iter().copied().nth(act_id_idx).unwrap();

                        ActiveOptions { act_id }
                    }
                }
            }

            ActiveOptions { act_id } => {
                match prompt(
                    PromptArgs {
                        str: cs![Active(game.state.act(act_id).type_)],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    [
                        (true, cs!["просмотреть"]),
                        (game.can_use_in_any_way(act_id), cs!["использовать"]),
                    ]
                    .into_iter(),
                ) {
                    None => input_state.previous(),
                    Some(0) => CheckActive { act_id },
                    Some(1) => UseActive { act_id },
                    _ => unreachable!(),
                }
            }

            CheckActive { act_id } => {
                println!(
                    "{}",
                    GameFormatted {
                        game,
                        value: act_id
                    }
                );

                input_state.previous()
            }

            UseActive { act_id } => {
                let can_use_on_field = game.can(
                    Event::Use {
                        act_id,
                        use_way: UseWay::OnField,
                    }
                    .sign(player_id),
                );

                let can_use_on_own_chr = game.can_use_on_own_chr(act_id);
                let can_use_on_enemy_chr = game.can_use_on_enemy_chr(act_id);

                match prompt(
                    PromptArgs {
                        str: cs!["использовать активку"],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    [
                        (can_use_on_field, cs!["на поле"]),
                        (can_use_on_own_chr, cs!["на своего персонажа"]),
                        (can_use_on_enemy_chr, cs!["на противника"]),
                    ]
                    .into_iter(),
                ) {
                    None => input_state.previous(),
                    Some(0) => UseActiveOnField { act_id },
                    Some(1) => UseActiveOnOwnCharacter { act_id },
                    Some(2) => UseActiveOnEnemyCharacter { act_id },
                    _ => unreachable!(),
                }
            }

            UseActiveOnField { act_id } => {
                Event::Use {
                    act_id,
                    use_way: UseWay::OnField,
                }
                .sign(player_id)
                .try_(game)
                .unwrap();

                println!(
                    "активка {} использована на поле",
                    cs![Active(game.state.act(act_id).type_)]
                );

                InputState::default()
            }

            UseActiveOnOwnCharacter { act_id } => {
                let target_id = game.state.chrs_on_field(player_id).next().unwrap();

                Event::Use {
                    act_id,
                    use_way: UseWay::OnCharacter(target_id),
                }
                .sign(player_id)
                .try_(game)
                .unwrap();

                println!(
                    "активка {} использована на персонажа {}",
                    cs![Active(game.state.act(act_id).type_)],
                    cs![Character(game.state.chr(target_id).type_)],
                );

                InputState::default()
            }

            UseActiveOnEnemyCharacter { act_id } => {
                let target_id = game.state.chrs_on_field(enemy_player_id).next().unwrap();
                Event::Use {
                    act_id,
                    use_way: UseWay::OnCharacter(target_id),
                }
                .sign(player_id)
                .try_(game)
                .unwrap();

                println!(
                    "активка {} использована на персонажа {}",
                    cs![Active(game.state.act(act_id).type_)],
                    cs![Character(game.state.chr(target_id).type_)],
                );

                InputState::default()
            }

            EndSubturn => {
                game.end_subturn(player_id);

                println!("подход завершён");

                InputState::default()
            }
        };

        println!();
    }
}
