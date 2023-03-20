pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БАЯН"]
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
        Condition(cs!["использован на персонажа"]),
        NamedPoint(
            cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""],
            cs![Damage, " -= 1"]
        ),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|mut game, args| {
            game.stat_add(args.target_id, StatType::Damage, 1);
            args
        }),

        ..Default::default()
    }
}
