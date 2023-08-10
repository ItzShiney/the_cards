pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["МОЙ РОТ РАЗВОРОТ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Мемы],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано в начале своего хода"]),
        Point(cs!["меняет направление ходов на противоположное"]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    todo!()
}
