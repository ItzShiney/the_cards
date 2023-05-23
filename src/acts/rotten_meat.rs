pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ROTTEN MEAT"]
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
        Condition(cs!["использовано на персонажа"]),
        Point(cs![Physique, " += 1, ", Vitality, " += 2"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Physique, 1).try_(game);
            _ = StatAdd::new(args.target_id, StatType::Vitality, 2).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
