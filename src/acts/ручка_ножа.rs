use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["РУЧКА НОЖА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использовано на персонажа"]),
        Point(cs![Physique, " += 1"]),
        Point(cs!["если ранее было использовано ", ЛезвиеНожа, ", получи ", Нож]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|mut game, args| {
            game.stat_add(args.target_id, StatType::Physique, 1);

            #[allow(unreachable_code)]
            #[allow(clippy::diverging_sub_expression)]
            if todo!("ранее было использовано ЛЕЗВИЕ НОЖА") {
                let owner_id = game.state.find_owner_of_act(args.act_id);

                let drawn_chr_id = game.state.chrs.add(CharacterInfo::new(Нож));
                game.state.chrs.add_to_player(drawn_chr_id, owner_id);
            }

            args
        }),

        ..Default::default()
    }
}
