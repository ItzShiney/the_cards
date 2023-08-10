pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ЖЁЛТАЯ ИСКРА"]
}

// D, потому что работает только после активок типа "наносит урон", а таких мало
#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Undertale]
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        Point(cs![Vitality, " = ", Physique]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let phy = game.stat(chr_id, StatType::Vitality, act_id);

    Event::stat_change(chr_id, StatType::Vitality, StatChange::Set(phy))
        .sign(act_id)
        .try_(game)?;

    Ok(chr_id)
}
