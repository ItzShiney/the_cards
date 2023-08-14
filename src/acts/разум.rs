pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["РАЗУМ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(cs!["\"I KNOW ALL\""], cs![Intellect, " += 2"]),
        Point(cs![
            "уже были использованы ",
            Тело,
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
    let stat_change = Event::stat_change(chr_id, StatType::Intellect, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    // TODO: game.state.was_used(...)
    if false {
        let owner_id = game.state.owner_id(act_id);

        let drawn_act_id = game.state.acts.add(ActiveInfo::new(Godhead));
        game.state.acts.add_to_player(drawn_act_id, owner_id);
    } else {
        stat_change?;
    }

    Ok(chr_id)
}
