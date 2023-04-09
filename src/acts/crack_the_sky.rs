pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["CRACK THE SKY"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        Point(cs!["наносит ", Random(cs!["0"]..=cs!["5"]), " ", Damage]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            let dmg = game.random(0, 5);
            _ = game.try_get_hurt(args.target_id, dmg);
            args
        }),

        ..Default::default()
    }
}
