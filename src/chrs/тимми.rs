pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ТИММИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [Мужчина],
        tags: [SouthPark],
    }.into()
}

// 1/0/-5
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(1),
        dmg!(0),
        int!(0),
    )
}

pub fn description() -> CustomString {
    cs![Epitaph(cs!["\"тимми тимми тимми\""])]
}
