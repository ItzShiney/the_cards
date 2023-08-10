pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ПИТОН"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByЛёня,
        genders: [],
        tags: [ЯзыкиПрограммирования],
    }.into()
}

// 2/3/-0
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(5), // народная любовь
        dmg!(9), // больно от того, насколько он плох местами
        int!(3),
    )
}

// TODO
// • удары дизморалят
