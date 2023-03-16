pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["РИКА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [Женщина],
        tags: [Higurashi],
    }.into()
}

// 1/1/-1
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(2),
        dmg!(2),
        int!(6),
    )
}
