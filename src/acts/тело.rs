pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ТЕЛО"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(
            cs!["\"I FEEL ALL\""],
            cs![Physique, " & ", Vitality, " += 2"]
        ),
        Point(cs![
            "уже были использованы ",
            Разум,
            " и ",
            Душа,
            " ",
            Implies,
            " получи ",
            Godhead
        ]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let phy_change = Event::stat_change(chr_id, StatType::Physique, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    let vit_change = Event::stat_change(chr_id, StatType::Vitality, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    // TODO: game.state.was_used(...)
    if false {
        let owner_id = game.state.owner_id(act_id);

        let drawn_act_id = game.state.acts.add(ActiveInfo::new(Godhead));
        game.state.acts.add_to_player(drawn_act_id, owner_id);
    } else {
        phy_change.or(vit_change)?;
    }

    Ok(chr_id)
}
