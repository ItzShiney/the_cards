pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БЬЯРНЕ СТРАУСТРУП"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: S,
        author: ByМаксим,
        genders: [Мужчина],
        tags: [Программирование],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(6),
        dmg!(9),
        int!(9 = const),
    )
}

pub fn description() -> CustomString {
    cs![
        NamedPoint(cs!["ВЕЧНЫЙ ПРОГРАММИСТ"], cs![Const(cs![Intellect])]),
        __,
        Condition(cs!["закончилась битва"]),
        NamedPoint(
            cs!["ZERO COST ABSTRACTIONS"],
            cs!["использованные на него владельцем активки возвращаются в руку"],
        ),
        __,
        Condition(cs!["противник умер"]),
        NamedPoint(cs!["BEST PRACTICES"], cs!["его тир..."]),
        Tab,
        Point(cs!["C, D ", Implies, " уничтожается"]),
        Tab,
        Point(cs!["A, B ", Implies, " переходит владельцу этой карты"]),
        __,
        Condition(cs!["противник атакует"]),
        NamedPoint(cs!["C++"], cs![Intellect, " противника..."]),
        Tab,
        Point(cs!["0..3 ", Implies, " атакует сам себя"]),
        Tab,
        Point(cs!["4..6 ", Implies, " не атакует"]),
    ]
}

// TODO
pub fn abilities() -> GameCallbacks {
    GameCallbacks::default()
}
