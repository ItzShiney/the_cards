use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["КОЗЕРОГ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Зодиак],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        NamedPoint(
            cs!["\"ALL STATS UP\""],
            cs![
                Physique, " & ", Vitality, " & ", Defence, " & ", Damage, " & ", Intellect, " += 2"
            ]
        ),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Physique, 2).try_(game);
            _ = StatAdd::new(args.target_id, StatType::Vitality, 2).try_(game);
            _ = StatAdd::new(args.target_id, StatType::Defence, 2).try_(game);
            _ = StatAdd::new(args.target_id, StatType::Damage, 2).try_(game);
            _ = StatAdd::new(args.target_id, StatType::Intellect, 2).try_(game);
            (args, ())
        }),

        ..Default::default()
    }
}
