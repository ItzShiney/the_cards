pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["КОММУНИЗМ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: S, 
        author: ByКостя, 
        genders: [], 
        tags: [ОбщественныйСтрой],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использован в качестве своего хода"]),
        Point(cs!["каждый игрок передаёт свою колоду следующему по направлению ходов"]),
        Point(cs!["эта карта уничтожается"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_field: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
