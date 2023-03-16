pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ПЛАНЯ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [Женщина],
        tags: [WePlanet],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 3/3/-4
    Stats::new(
        phy!(4),
        dmg!(4),
        int!(2),
    )
}

pub fn description() -> CustomString {
    cs![
        // TODO
        Condition(cs!["выставлена"]),
        NamedPoint(
            cs!["КРИНЖ И ПЕНИЕ"],
            cs![Intellect, " случайного персонажа в колоде противника -= 1"]
        ),
        __,
        // TODO
        Condition(cs!["пока на поле"]),
        NamedPoint(
            cs!["МАКСИМАЛЬНАЯ СПЛЮЩЕННОСТЬ"],
            cs![Intellect, " всех персонажей на поле меньше на 4"]
        ),
        __,
        // TODO
        Condition(cs!["персонаж из биты вернулся к владельцу"]),
        NamedPoint(cs!["\"ВЕРНИ САНКИ\""], cs![Physique, " всех персонажей в руке += 2"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks { ..Default::default() }
}
