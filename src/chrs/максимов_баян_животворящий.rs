pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МАКСИМОВ БАЯН ЖИВОТВОРЯЩИЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [Животворит],
    }.into()
}

// 4/1/-0
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(6),
        dmg!(3),
        int!(8),
    )
}
