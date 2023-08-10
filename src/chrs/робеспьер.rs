pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["РОБЕСПЬЕР"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [Мужчина],
        tags: [Реальность],
    }.into()
}

// 2/5/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(5),
        dmg!(5),
        int!(5),
    )
}

pub fn description() -> CustomString {
    cs![Epitaph(cs!["\"vive la révolution\""]),]
}
