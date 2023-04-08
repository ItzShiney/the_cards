pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["THE SUN"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Таро],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        Point(cs![Vitality, " = ", Physique]),
        Point(cs!["возьми персонажа и активку из стопки добора"]),
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
