pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ZODIAC"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Зодиак],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        Point(cs![Vitality, " += [кол-во использованных ", Зодиак, "-активок за игру]"]),
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
