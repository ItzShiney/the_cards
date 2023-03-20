pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ГЛАЗ КТУЛХУ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByМаксим,
        genders: [],
        tags: [Terraria],
    }.into()
}

// 4/3/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(8),
        dmg!(6),
        int!(2),
    )
}

pub fn description() -> CustomString {
    cs![NamedPoint(
        cs!["\"ТАРАНИТ... ИНОГДА\""],
        cs!["с шансом 1/2 наносит на 1 ", Damage, " больше"]
    ),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_attack: Some(|game, mut args| {
            if game.random_bool(1. / 2.) {
                args.dmg += 1;
            }

            args
        }),

        ..Default::default()
    }
}
