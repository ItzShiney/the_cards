pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["МАРИО"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [Мужчина],
        tags: [],
    }.into()
}

// 2/2/-3
#[rustfmt::skip] pub fn stats() -> Stats {
    Stats::new(
        phy!(5),
        dmg!(5),
        int!(6),
    )
}

pub fn description() -> CustomString {
    cs![
        Activatable,
        Condition(cs!["битва"]),
        NamedPoint(cs!["ПРЫЖОК НА ЛИЦО"], cs![Vitality, " противника /= 2"]),
    ]
}
