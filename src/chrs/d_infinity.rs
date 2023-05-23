pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["D INFINITY"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 0/0/-0
    Stats::new(
        phy!(3?),
        dmg!(3?),
        int!(3?),
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["выставлен"]),
        Point(cs![
            Physique,
            " & ",
            Vitality,
            ", ",
            Damage,
            ", ",
            Intellect,
            " = ",
            Random(cs!["0"]..=cs!["9"]),
        ]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_place: Some(|game, args| {
            let self_id = args.chr_id;

            let phy_vit = game.random(0, 9);
            let dmg = game.random(0, 9);
            let int = game.random(0, 9);

            game.force_set_phy_vit(self_id, phy_vit);
            game.force_set_stat(self_id, StatType::Damage, dmg);
            game.force_set_stat(self_id, StatType::Intellect, int);

            (args, ())
        }),

        ..Default::default()
    }
}
