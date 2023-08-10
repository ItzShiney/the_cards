pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["АЙОЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [Женщина],
        tags: [Nichijou],
    }.into()
}

// 3/1/-inf
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(4),
        dmg!(1),
        int!(0 = const),
    )
}

pub fn description() -> CustomString {
    cs![Epitaph(cs![
        "\"супер-ультра-громадно-восхитительно-невероятно-непонятно\""
    ])]
}
