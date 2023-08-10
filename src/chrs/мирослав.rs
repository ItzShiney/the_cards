pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["МИРОСЛАВ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [Мужчина],
        tags: [Реальность],
    }.into()
}

// 2/2/-4
#[rustfmt::skip] pub fn stats() -> Stats {
    Stats::new(
        phy!(3),
        dmg!(4),
        int!(0),
    )
}
