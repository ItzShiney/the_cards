use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ПУНКТ ПЯТЫЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [Скрытая, Portal],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(
            cs!["\"ЗАМИНИРОВАТЬ КНОПКУ\""],
            cs!["противник использовал активку ", Implies, " наносит 4 ", Damage]
        ),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
