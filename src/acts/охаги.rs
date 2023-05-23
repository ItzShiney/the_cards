pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ОХАГИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [],
        tags: [Higurashi],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использованы на персонажа с ", Intellect, " ", LE, " 3"]),
        Point(cs!["наносят 1 ", Damage]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        can_use_on_chr: Some(|game, args| {
            let chr_int = game.state.chr(args.target_id).stats.int.0;
            (chr_int <= 3).then_some(args)
        }),

        force_use_on_chr: Some(|game, args| {
            _ = GetHurt::new(args.target_id, 1).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
