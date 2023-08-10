pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ЧЁРТ 480"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [Скрытая, ПепежноеСущество, ЦитатыКости],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано в битве"]),
        Point(cs![
            "следующая активка, использованная противником, не сработает"
        ]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    todo!()
}
