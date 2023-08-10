pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ОХАГИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [],
        tags: [Higurashi],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs![
            "использованы на персонажа с ",
            Intellect,
            " ",
            LE,
            " 3"
        ]),
        Point(cs!["наносят 1 ", Damage]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let int = game.stat(chr_id, StatType::Intellect, act_id);
    if !(int <= 3) {
        return Err(Cancelled);
    }

    Event::get_hurt(chr_id, 1).sign(act_id).try_(game)?;

    Ok(chr_id)
}
