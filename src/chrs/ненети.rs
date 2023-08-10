pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["Н\u{0301}ЕНЕТИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [Женщина],
        tags: [NewGame],
    }.into()
}

// 5/1/-1
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(7),
        dmg!(1),
        int!(4),
    )
}
