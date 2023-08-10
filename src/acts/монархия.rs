pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["МОНАРХИЯ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована в ответ на ", Коммунизм]),
        Point(cs!["отменяет его эффект"]),
        Point(cs!["эта карта уничтожается"]),
    ]
}
