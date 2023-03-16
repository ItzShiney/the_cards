pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["САТОКА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [Женщина],
        tags: [Higurashi],
    }.into()
}

// 3/2/-4
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(5), // терпит много лещей
        dmg!(3),
        int!(7), // ловушками перебивает спецотряд
    )
}
