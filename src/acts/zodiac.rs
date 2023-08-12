pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ZODIAC"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Зодиак],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        Point(cs![
            Vitality,
            " += [кол-во использованных ",
            Зодиак,
            "-активок за игру]"
        ]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let used_acts_count = game
        .state
        .events_flatten()
        .filter_map(|event| match event.value {
            Event::Use { act_id, .. } => {
                if game.state.act(act_id).type_.groups().contains(&Зодиак) {
                    Some(act_id)
                } else {
                    None
                }
            }

            _ => None,
        })
        .unique()
        .count();

    Event::stat_change(
        chr_id,
        StatType::Vitality,
        StatChange::Add(used_acts_count as _),
    )
    .sign(act_id)
    .try_(game)?;

    Ok(chr_id)
}
