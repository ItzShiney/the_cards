pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЖЁЛТАЯ ИСКРА"]
}

// D, потому что работает только после активок типа "наносит урон", а таких мало
#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Undertale]
    }.into()
}

pub fn description() -> CustomString {
    cs![Condition(cs!["использована на персонажа"]), Point(cs![Vitality, " = ", Physique]),]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|mut game, args| {
            let phy = game.state.chr(args.target_id).stats.phy.0.into_value();
            game.force_set_stat(args.target_id, StatType::Vitality, phy);

            args
        }),

        ..Default::default()
    }
}
