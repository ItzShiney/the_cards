pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["НЕДОСЫП"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность, Дизморалит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        Point(cs![Vitality, " & ", Intellect, " -= 2"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(-2))
        .sign(act_id)
        .try_(game);

    let int = Event::stat_change(chr_id, StatType::Intellect, StatChange::Add(-2))
        .sign(act_id)
        .try_(game);

    vit.or(int)?;
    Ok(chr_id)
}
