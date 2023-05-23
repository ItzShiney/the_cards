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
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Damage, 2).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
