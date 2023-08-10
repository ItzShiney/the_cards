pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ХРИВНА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [],
        tags: [Реальность],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        Point(cs![Intellect, " -= 1"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    _ = Event::stat_change(chr_id, StatType::Intellect, StatChange::Add(-1))
        .sign(act_id)
        .try_(game);

    Ok(chr_id)
}
