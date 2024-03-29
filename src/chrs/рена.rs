pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["РЕНА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [Женщина],
        tags: [Higurashi],
    }.into()
}

// 2/3/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(4),
        dmg!(7),
        int!(6),
    )
}
