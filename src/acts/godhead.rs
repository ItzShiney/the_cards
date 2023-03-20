use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["GODHEAD"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(cs!["\"GOD TEARS\""], cs![Damage, " += 2"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Damage, 2);

            args
        }),

        ..Default::default()
    }
}
