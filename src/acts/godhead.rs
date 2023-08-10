pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["GODHEAD"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(cs!["\"GOD TEARS\""], cs![Damage, " += 2"]),
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
