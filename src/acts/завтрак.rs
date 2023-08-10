pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ЗАВТРАК"]
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
        Condition(cs!["использован на персонажа"]),
        Point(cs![
            Physique,
            " += ",
            Choice(vec![cs!["-1"], cs!["0"], cs!["2"]])
        ]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(2))
        .sign(act_id)
        .try_(game)?;

    Ok(chr_id)
}
