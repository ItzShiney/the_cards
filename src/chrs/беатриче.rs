pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БЕАТРИЧЕ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByМаксим,
        genders: [Женщина],
        tags: [Umineko, Иллюзия],
    }.into()
}

// 1/4/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(5),
        dmg!(8),
        int!(7),
    )
}

pub fn description() -> CustomString {
    cs![Condition(cs!["умерла"]), Point(cs!["с шансом 1/4 возвращается в руку"])]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        die: Some(
            |game, args| {
                if game.random_bool(1. / 4.) {
                    Break(Err(Terminated))
                } else {
                    Continue(args)
                }
            },
        ),

        ..Default::default()
    }
}
