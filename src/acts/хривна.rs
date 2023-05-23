pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ХРИВНА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [],
        tags: [Реальность],
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использована на персонажа"]), Point(cs![Intellect, " -= 1"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Intellect, -1).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
