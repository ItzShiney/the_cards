pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БАНКА С ВАРЕНЬЕМ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 1/3/-0
    Stats::new(
        phy!(1), // легко разбивается
        dmg!(2), // бьёт осколками
        int!(0),
    )
}

pub fn description() -> CustomString {
    cs![Point(cs!["не атакует, если ", Intellect, " противника ", GE, " 3"])]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        attack_map: Some(|game, args| {
            if game.state().chr(args.attacker_id).stats.int.0.into_value() >= 3 {
                Break(Err(Terminated))
            } else {
                Continue(args)
            }
        }),

        ..Default::default()
    }
}
