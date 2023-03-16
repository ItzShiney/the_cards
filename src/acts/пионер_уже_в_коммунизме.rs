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
        Condition(cs!["использовано на карту в руке"]),
        Point(cs!["отдай её следующему по направлению ходов игроку"]),
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
