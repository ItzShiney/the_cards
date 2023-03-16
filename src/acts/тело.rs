use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ТЕЛО"]
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
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(cs!["\"I FEEL ALL\""], cs![Physique, " & ", Vitality, " += 2"]),
        Point(cs!["уже были использованы ", Разум, " и ", Душа, " ", Implies, " получи ", Godhead]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Physique, 2);
            game.stat_add(args.target_id, StatType::Vitality, 2);
            Continue(args)
        }),

        ..Default::default()
    }
}
