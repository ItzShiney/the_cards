pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["НЕУТЕШИТЕЛЬНЫЙ ПРИЗ"]
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
        Epitaph(cs![
            "максим писал про эту медаль так:\n",
            "\"пепега какая-то\""
        ]),
        __,
        Condition(cs!["использован на персонажа"]),
        Point(cs!["статы, равные максимальному -= 1"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    todo!()
}
