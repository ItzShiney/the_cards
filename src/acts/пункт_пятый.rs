pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ПУНКТ ПЯТЫЙ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [Скрытая, Portal],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован на персонажа"]),
        NamedPoint(
            cs!["\"ЗАМИНИРОВАТЬ КНОПКУ\""],
            cs![
                "противник использовал активку ",
                Implies,
                " наносит 4 ",
                Damage
            ]
        ),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    todo!()
}
