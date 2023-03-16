pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ДУХ ТВОЕЙ КВАРТИРЫ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByКостя,
        genders: [Женщина],
        tags: [],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 4/3/-4
    Stats::new(
        phy!(8),
        dmg!(5),
        int!(1),
    )
}

pub fn description() -> CustomString {
    cs![
        Epitaph(cs!["\"твоё личное бревно\""]),
        __,
        Condition(cs!["пока у владельца ", LE, " 2 персонажей"]),
        Point(cs![Damage, " больше на 2"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks { ..Default::default() }
}
