pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ROTTEN MEAT"]
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
        Point(cs![Physique, " += 1, ", Vitality, " += 2"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let phy = Event::stat_change(chr_id, StatType::Physique, StatChange::Add(1))
        .sign(act_id)
        .try_(game);

    let vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    phy.or(vit)?;
    Ok(chr_id)
}
