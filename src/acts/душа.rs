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
        use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Defence, 2);
            Continue(args)
        }),

        ..Default::default()
    }
}
