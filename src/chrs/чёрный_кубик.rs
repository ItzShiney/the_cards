pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЧЁРНЫЙ КУБИК"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByМаксим,
        genders: [],
        tags: [],
    }.into()
}

// 3/1/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(3),
        dmg!(1),
        int!(5),
    )
}
