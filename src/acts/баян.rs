pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["БАЯН"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByМаксим,
        genders: [],
        tags: [Дизморалит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(
            cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""],
            cs![Damage, " -= 1"]
        ),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    _ = Event::stat_change(chr_id, StatType::Damage, StatChange::Add(1))
        .sign(act_id)
        .try_(game);

    Ok(chr_id)
}
