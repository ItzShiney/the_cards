pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["О,БРАТКА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder { 
        tier: A, 
        author: ByЛёша, 
        genders: [], 
        tags: [],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs![
            "использована на противника & твой персонаж не выставлен"
        ]),
        Point(cs![
            "персонаж противника становится твоим и выставляется от тебя"
        ]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    todo!()
}
