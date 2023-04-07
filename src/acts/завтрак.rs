pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЗАВТРАК"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        Point(cs![Physique, " += ", Choice(vec![cs!["-1"], cs!["0"], cs!["2"]])]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Vitality, 2);
            args
        }),

        ..Default::default()
    }
}
