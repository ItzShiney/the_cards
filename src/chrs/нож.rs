pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["НОЖ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Нераздаваемая],
    }.into()
}

// 2/?/-0
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(3),
        dmg!(5?),
        int!(1),
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["выставлен"]),
        Point(cs![Damage, " = ", Sum { times: cs!["9"], body: cs![Random(cs!["0"]..=cs!["1"])] }]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        post_place: Some(|game, args| {
            let value = repeat_with(|| game.random(0, 1)).take(9).sum();

            let self_id = args.chr_id;
            game.force_set_stat(self_id, StatType::Damage, value);
        }),

        ..Default::default()
    }
}
