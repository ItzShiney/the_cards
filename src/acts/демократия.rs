pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ДЕМОКРАТИЯ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: S, 
        author: ByЛёня, 
        genders: [],
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована в качестве своего хода"]),
        Point(cs!["[кол-во игроков] карт генерируются и случайно распределяются между игроками"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        can_use_on_field: Some(|_game, _args| {
            todo!();
        }),

        force_use_on_field: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
