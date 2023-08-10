pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ДЕМОКРАТИЯ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: S, 
        author: ByЛёня, 
        genders: [],
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована в качестве своего хода"]),
        Point(cs![
            "[кол-во игроков] карт генерируются и случайно распределяются между игроками"
        ]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    todo!()
}
