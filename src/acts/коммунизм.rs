pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["КОММУНИЗМ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: S, 
        author: ByКостя, 
        genders: [], 
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован в качестве своего хода"]),
        Point(cs![
            "каждый игрок передаёт свою колоду следующему по направлению ходов"
        ]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    todo!()
}
