pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ТУПОСТЬ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Моралит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        // TODO
        Condition(cs!["использована в ответ на ", Дизморалит, "-активку"]),
        Point(cs!["отменяет её эффект"]),
    ]
}
