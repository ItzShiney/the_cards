pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["САТОКИНА БИТА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [],
        tags: [Higurashi],
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использована на персонажа"]), Point(cs![Damage, " += 2"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_chr: Some(|game, args| {
            game.stat_add(args.target_id, StatType::Damage, 2);
            Continue(args)
        }),

        ..Default::default()
    }
}
