pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["САТОКИНА БИТА"]
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
        Condition(cs!["использована на персонажа"]),
        Point(cs![Damage, " += 2"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    Event::stat_change(chr_id, StatType::Damage, StatChange::Add(2))
        .sign(act_id)
        .try_(game)?;

    Ok(chr_id)
}
