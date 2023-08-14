pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["THE SUN"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Таро],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на персонажа"]),
        Point(cs![Vitality, " = ", Physique]),
        Point(cs!["возьми персонажа и активку из стопки добора"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let phy = game.stat(chr_id, StatType::Physique, act_id);
    let set_vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Set(phy))
        .sign(act_id)
        .try_(game);

    let owner_id = game.state.owner_id(act_id);
    let take_chr = Event::take_chr(owner_id).sign(act_id).try_(game);
    let take_act = Event::take_act(owner_id).sign(act_id).try_(game);

    set_vit.or(take_chr).or(take_act)?;
    Ok(chr_id)
}
