pub use crate::card_uses::*;

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

pub fn abilities() -> GameCallbacks {
    GameCallbacks { force_use_on_chr: Some(|_game, _args| todo!()), ..Default::default() }
}
