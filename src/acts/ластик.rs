pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ЛАСТИК"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано в качестве хода"]),
        Point(cs![
            "уничтожает все карты в бите и по одной выбранной каждым игроком у себя в руке"
        ]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    todo!()
}
