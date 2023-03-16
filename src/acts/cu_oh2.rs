pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["CU(OH)₂"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [Химия],
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использован на персонажа"]), Point(cs![Vitality, " /= 2"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks { use_on_chr: Some(|_game, _args| todo!()), ..Default::default() }
}
