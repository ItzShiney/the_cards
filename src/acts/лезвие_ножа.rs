pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ЛЕЗВИЕ НОЖА"]
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
        Condition(cs!["использовано на персонажа"]),
        Point(cs![Damage, " += 1"]),
        Point(cs![
            "если ранее была использована ",
            РучкаНожа,
            ", получи ",
            Нож
        ]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let stat_change = Event::stat_change(chr_id, StatType::Damage, StatChange::Add(1))
        .sign(act_id)
        .try_(game);

    // TODO: game.state.was_used(...)
    if false {
        let owner_id = game.state.find_owner_of_act(act_id);

        let drawn_chr_id = game.state.chrs.add(CharacterInfo::new(Нож));
        game.state.chrs.add_to_player(drawn_chr_id, owner_id);
    } else {
        stat_change?;
    }

    Ok(chr_id)
}
