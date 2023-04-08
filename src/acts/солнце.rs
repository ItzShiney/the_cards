pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["СОЛНЦЕ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность, Моралит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        Point(cs![Vitality, " += 1"]),
        Point(cs!["персонаж — ", Растение, " ", Implies]),
        Tab,
        NamedPoint(cs!["ФОТОСИНТЕЗ"], cs![Damage, " += 3"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        can_use_on_chr: Some(|_game, _args| {
            todo!();
        }),

        force_use_on_chr: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
