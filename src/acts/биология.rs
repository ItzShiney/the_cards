pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["НЕДОСЫП"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность, Дизморалит],
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использован на персонажа"]), Point(cs![Vitality, " & ", Intellect, " -= 2"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Vitality, -2);
            game.stat_add(args.target_id, StatType::Intellect, -2);
            args
        }),

        ..Default::default()
    }
}
