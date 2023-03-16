pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ТУПОСТЬ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Моралит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована в ответ на ", Дизморалит, "-активку"]),
        Point(cs!["отменяет её эффект"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_field: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
