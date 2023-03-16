pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЗЕРКАЛО"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёша,
        genders: [],
        tags: [Реальность],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        // TODO
        // (нужна какая-то пометка, какие способности возможно копировать)
        Condition(cs!["использовано на персонажа"]),
        Point(cs!["копирует выбранную способность противника"]),
    ]
}
