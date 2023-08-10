pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["УТЕШИТЕЛЬНЫЙ ПРИЗ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Моралит],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Epitaph(cs![
            "толстой писал про эту медаль так:\n",
            "\"всевеликая всероссийская посеребрённая золотистая с платиновым отблеском заточенная\n",
            "медаль победителя всевеликого всероссийского этапа всевеликой всероссийской олимпиады\n",
            "всевеликих всероссийских школьников по всевеликому всеросскийскому животворящему программированию\""]),
        __,
        Condition(cs!["использован на персонажа"]),
        Point(cs!["статы, равные минимальному += 1"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    todo!()
}
