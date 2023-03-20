use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["\"ЛЕЖИТ ПИОНЕР БЕЗ ПРИЗНАКОВ ЖИЗНИ, ЕМУ ХОРОШО, ОН УЖЕ В КОММУНИЗМЕ\""]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByКостя,
        genders: [],
        tags: [Цитаты],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано"]),
        Point(cs!["выбери карту в руке. отдай её следующему по направлению ходов игроку"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_field: Some(|_game, _args| {
            todo!();
        }),

        ..Default::default()
    }
}
