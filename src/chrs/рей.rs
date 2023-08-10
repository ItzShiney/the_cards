pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["РЕЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [Мужчина],
        tags: [],
    }.into()
}

// 1/3/-2
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(2),
        dmg!(5),
        int!(6),
    )
}
