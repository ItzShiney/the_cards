pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ТАКАНО"]
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

// 4/4/-1
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(4),
        dmg!(4),
        int!(7),
    )
}
