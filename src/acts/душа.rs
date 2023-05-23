use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ДУША"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        NamedPoint(cs!["\"I AM ALL\""], cs![Defence, " += 2"]),
        Point(cs!["уже были использованы ", Разум, " и ", Тело, " ", Implies, " получи ", Godhead]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Defence, 2).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
