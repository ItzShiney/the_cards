pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МАКСИМ ЖИВОТВОРЯЩИЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: A,
        author: ByЛёня,
        genders: [Мужчина],
        tags: [Максим],
    }.into()
}

// 5/5/-4
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(9),
        dmg!(9),
        int!(3),
    )
}

pub fn description() -> CustomString {
    // TODO
    cs![
        "активируемые способности раз за игру:\n",
        Point(cs!["\x1b[1m[BLUE CANDLE]\x1b[0m"]),   // TODO
        Point(cs!["\x1b[1m[LIBRA]\x1b[0m"]),         // TODO
        Point(cs!["\x1b[1m[CROOKED PENNY]\x1b[0m"]), // TODO
        Point(cs![Godhead]),
        __,
        Condition(cs!["в игре использована ", Животворит, "-активка"]),
        NamedPoint(cs!["ЖИВОТВОРЯЩАЯ ЖИВОТВОРЯЩЕСТЬ"], cs![Intellect, " += 2"]),
    ]
}
