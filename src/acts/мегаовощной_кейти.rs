pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["МЕГАОВОЩНОЙ КЕЙТИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [],
        tags: [Higurashi],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        Point(cs![Intellect, " = 0"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    Event::stat_change(chr_id, StatType::Intellect, StatChange::Set(0))
        .sign(act_id)
        .try_(game)?;

    Ok(chr_id)
}
