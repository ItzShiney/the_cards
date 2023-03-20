pub use crate::card_uses::*;

// арт — уголёк

pub fn name() -> CustomString {
    cs!["НЕУТЕШИТЕЛЬНЫЙ ПРИЗ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByМаксим,
        genders: [],
        tags: [Дизморалит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Epitaph(cs!["максим писал про эту медаль так:\n", "\"пепега какая-то\""]),
        __,
        Condition(cs!["использован на персонажа"]),
        Point(cs!["статы, равные максимальному -= 1"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks { force_use_on_chr: Some(|_game, _args| todo!()), ..Default::default() }
}
