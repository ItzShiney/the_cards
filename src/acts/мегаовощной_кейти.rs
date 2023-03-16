pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["МЕГАОВОЩНОЙ КЕЙТИ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [],
        tags: [Higurashi],
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использован на персонажа"]), Point(cs![Intellect, " = 0"]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_chr: Some(|_game, _args| todo!("{} = 0", cs![Intellect])),

        ..Default::default()
    }
}
