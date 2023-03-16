pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МАГДАЛИНА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [Женщина],
        tags: [TBoI],
    }.into()
}

// 4/1/-2
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(7),
        dmg!(2),
        int!(6), // TODO брать у CharacterType::Айзек
    )
}

pub fn description() -> CustomString {
    cs![Activatable, NamedPoint(cs!["НЯМ СЕРДЦЕ"], cs![Vitality, " += 2"]),]
}
