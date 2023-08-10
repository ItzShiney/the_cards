pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["CRACK THE SKY"]
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
        Point(cs!["наносит ", Random(cs!["0"]..=cs!["5"]), " ", Damage]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let Event::Random {
        output: Some(dmg), ..
    } = Event::random(0, 5).sign(act_id).try_(game)?.value
    else {
        unreachable!()
    };

    Event::get_hurt(chr_id, dmg).sign(act_id).try_(game)?;

    Ok(chr_id)
}
