pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["КОСА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [Женщина],
        tags: [Реальность],
    }.into()
}

// 2/3/-1
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(3),
        dmg!(0),
        int!(8),
    )
}
