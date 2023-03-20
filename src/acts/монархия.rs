pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МОНАРХИЯ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована в ответ на ", Коммунизм]),
        Point(cs!["отменяет его эффект"]),
        Point(cs!["эта карта уничтожается"]),
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
