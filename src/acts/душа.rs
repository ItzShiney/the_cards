pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ДУША"]
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
        Condition(cs!["использована на персонажа"]),
        NamedPoint(cs!["\"I AM ALL\""], cs![Defence, " += 2"]),
        Point(cs![
            "уже были использованы ",
            Разум,
            " и ",
            Тело,
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
    let stat_change = Event::stat_change(chr_id, StatType::Defence, StatChange::Add(2))
        .sign(act_id)
        .try_(game);

    // TODO: game.state.was_used(...)
    if false {
        let owner_id = game.state.find_owner_of_act(act_id);

        let drawn_act_id = game.state.acts.add(ActiveInfo::new(Godhead));
        game.state.acts.add_to_player(drawn_act_id, owner_id);
    } else {
        stat_change?;
    }

    Ok(chr_id)
}
