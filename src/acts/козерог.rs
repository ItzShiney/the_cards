pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["КОЗЕРОГ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Зодиак],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(
            cs!["\"ALL STATS UP\""],
            cs![
                Physique, " & ", Vitality, " & ", Defence, " & ", Damage, " & ", Intellect, " += 2"
            ]
        ),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let phy = Event::stat_change(chr_id, StatType::Physique, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    let vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    let def = Event::stat_change(chr_id, StatType::Defence, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    let dmg = Event::stat_change(chr_id, StatType::Damage, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    let int = Event::stat_change(chr_id, StatType::Intellect, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    phy.or(vit).or(def).or(dmg).or(int)?;
    Ok(chr_id)
}
