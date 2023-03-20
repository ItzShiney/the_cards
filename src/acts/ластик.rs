pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЛАСТИК"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано в качестве хода"]),
        Point(cs!["уничтожает все карты в бите и по одной выбранной каждым игроком у себя в руке"]),
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
