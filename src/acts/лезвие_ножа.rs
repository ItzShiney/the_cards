use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ЛЕЗВИЕ НОЖА"]
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
        Point(cs![Damage, " += 1"]),
        Point(cs!["если ранее была использована ", РучкаНожа, ", получи ", Нож]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        force_use_on_chr: Some(|game, args| {
            _ = StatAdd::new(args.target_id, StatType::Physique, 1).try_(game);

            let was_ручка_ножа_used = false; // TODO ранее была использована РУЧКА НОЖА
            if was_ручка_ножа_used {
                let owner_id = game.state.find_owner_of_act(args.act_id);

                let drawn_chr_id = game.state.chrs.add(CharacterInfo::new(Нож));
                game.state.chrs.add_to_player(drawn_chr_id, owner_id);
            }

            (args, ())
        }),

        ..Default::default()
    }
}
