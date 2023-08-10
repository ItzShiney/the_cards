pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ТЕТРАДЬ СМЕРТИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder { 
        tier: B, 
        author: ByКостя, 
        genders: [], 
        tags: [DeathNote]
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        Point(cs!["мгновенно убивает его"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    Event::Die { chr_id }.sign(act_id).try_(game)?;

    Ok(chr_id)
}
