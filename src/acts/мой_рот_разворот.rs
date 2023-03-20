use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МОЙ РОТ РАЗВОРОТ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Мемы],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано в начале своего хода"]),
        Point(cs!["меняет направление ходов на противоположное"]),
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
