pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ТЕТРАДЬ СМЕРТИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder { 
        tier: B, 
        author: ByКостя, 
        genders: [], 
        tags: [DeathNote]
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использована на персонажа"]), Point(cs!["мгновенно убивает его"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = game.try_die(args.target_id);
            args
        }),

        ..Default::default()
    }
}
