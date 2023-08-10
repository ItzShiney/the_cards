pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ISAAC"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [Мужчина],
        tags: [TBoI],
    }.into()
}

// 2/1/-2
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(4),
        dmg!(3),
        int!(6),
    )
}
