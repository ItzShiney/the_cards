pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["0:18"]
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
        NamedPoint(
            cs!["\"AND THEY WERE BOTH ISAAC\""],
            cs!["превращает его в ", Isaac]
        ),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    if game.state.chr(chr_id).type_ == Isaac {
        return Err(Cancelled("[0:18]: chr was already [ISAAC]"));
    }

    Event::morph_chr(chr_id, CharacterInfo::new(Isaac))
        .sign(act_id)
        .try_(game)?;

    Ok(chr_id)
}
