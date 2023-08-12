pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["СОЛНЦЕ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность, Моралит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        Point(cs![Vitality, " += 1"]),
        Point(cs!["персонаж — ", Растение, " ", Implies]),
        Tab,
        NamedPoint(cs!["ФОТОСИНТЕЗ"], cs![Damage, " += 3"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(1))
        .sign(act_id)
        .try_(game);

    let photosynthesis = {
        let groups = game.state.chr(chr_id).type_.groups();
        if groups.contains(&Растение) {
            Event::stat_change(chr_id, StatType::Damage, StatChange::Add(3))
                .sign(act_id)
                .try_(game)
        } else {
            Err(Cancelled("[СОЛНЦЕ]: not a plant"))
        }
    };

    vit.or(photosynthesis)?;
    Ok(chr_id)
}
