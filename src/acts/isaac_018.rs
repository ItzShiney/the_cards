use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["0:18"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(cs!["\"AND THEY WERE BOTH ISAAC\""], cs!["превращает его в ", Isaac]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        can_use_on_chr: Some(|game, args| {
            (game.state.chr(args.target_id).type_ != Isaac).then_some(args)
        }),

        force_use_on_chr: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
