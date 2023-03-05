pub mod macro_;

use crate::{
    acts,
    group::Group,
    host::chain::Chain,
    {chrs::CharacterType, cs, custom_string::CustomString},
    {game_state::chr_info::CharacterInfo, host::GameCallbacks, stats::Stat},
};

use std::collections::BTreeSet;

acts! {
    // /*
    ПустаяКарта {
        name: cs!["ПУСТАЯ КАРТА"],
        groups: [Group::ByShiney, Group::TBoI],

        description: cs![
            Condition(cs!["использована"]),
            Point(cs!["выбери активку в руке. эта карта повторит эффект выбранной"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|game, args| {
                let owner_id = game.state().acts.find_owner(args.act_id);
                let imitated_act_id = game.choose_hand_act(owner_id);

                todo!("повторить эффект {:?}", imitated_act_id)
            }),

            ..Default::default()
        },
    }

    Баян {
        name: cs!["БАЯН"],
        groups: [Group::ByMaxvog, Group::Dismoral],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""], cs![Damage, " -= 3"]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                game.modify_stat(args.target_id, Stat::Damage, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ЖёлтаяИскра {
        name: cs!["ЖЁЛТАЯ ИСКРА"],
        groups: [Group::ByShiney, Group::Undertale],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs![Vitality, " = ", Physique]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                let phy = game.state().chr(args.target_id).stats.phy.0.into_value();
                game.force_set_stat(args.target_id, Stat::Vitality, phy);

                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ТетрадьСмерти {
        name: cs!["ТЕТРАДЬ СМЕРТИ"],
        groups: [Group::ByConstantine, Group::DeathNote],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs!["мгновенно убивает его"]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                let _ = game.die(args.target_id);
                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    Коммунизм {
        name: cs!["КОММУНИЗМ"],
        groups: [Group::ByConstantine, Group::SocialOrder],

        description: cs![
            Condition(cs!["использован в качестве своего хода"]),
            Point(cs!["каждый игрок передаёт свою колоду следующему по направлению ходов"]),
            Point(cs!["эта карта уничтожается"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!()
            }),

            ..Default::default()
        },
    }

    ОБратка {
        name: cs!["О,БРАТКА"],
        groups: [Group::ByZoinX],

        description: cs![
            Condition(cs!["использована на противника ", And, " твой персонаж не выставлен"]),
            Point(cs!["персонаж противника становится твоим и выставляется от тебя"]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                let owner_id = game.state().acts.try_find_owner(args.act_id);
                let target_owner_id = game.state().chrs.try_find_owner(args.target_id);

                if owner_id == target_owner_id {
                    return Chain::Break(Err(()));
                }

                todo!()
            }),

            ..Default::default()
        },
    }

    ЛезвиеНожа {
        name: cs!["ЛЕЗВИЕ НОЖА"],
        groups: [Group::ByShiney, Group::TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs![Damage, " += 1"]),
            Point(cs!["если ранее была использована ", РучкаНожа, ", получи ", Нож]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                game.modify_stat(args.target_id, Stat::Physique, 1);

                #[allow(unreachable_code)]
                if todo!("ранее была использована РУЧКА НОЖА") {
                    let owner_id = game.state().acts.find_owner(args.act_id);

                    let drawn_chr_id = game.state_mut().chrs.add(CharacterInfo::new(CharacterType::Нож));
                    game.state_mut().chrs.add_to_player(drawn_chr_id, owner_id);
                }

                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    РучкаНожа {
        name: cs!["РУЧКА НОЖА"],
        groups: [Group::ByShiney, Group::TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs![Physique, " += 1"]),
            Point(cs!["если ранее было использовано ", ЛезвиеНожа, ", получи ", Нож]),
        ],

        abilities: GameCallbacks {
            use_on_character: Some(|game, args| {
                game.modify_stat(args.target_id, Stat::Physique, 1);

                #[allow(unreachable_code)]
                if todo!("ранее было использовано ЛЕЗВИЕ НОЖА") {
                    let owner_id = game.state().acts.find_owner(args.act_id);

                    let drawn_chr_id = game.state_mut().chrs.add(CharacterInfo::new(CharacterType::Нож));
                    game.state_mut().chrs.add_to_player(drawn_chr_id, owner_id);
                }

                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }
    // */
}
