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
    todo!()
}
