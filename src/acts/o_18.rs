pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["0:18"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(
            cs!["\"AND THEY WERE BOTH ISAAC\""],
            cs!["превращает его в ", Isaac]
        ),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    todo!()
}
