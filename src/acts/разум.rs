use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["РАЗУМ"]
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
        Condition(cs!["использован на персонажа"]),
        NamedPoint(cs!["\"I KNOW ALL\""], cs![Intellect, " += 2"]),
        Point(cs!["уже были использованы ", Тело, " и ", Душа, " ", Implies, " получи ", Godhead]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Intellect, 2);
            args
        }),

        ..Default::default()
    }
}
