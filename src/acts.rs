mod _macro;

use crate::acts;
use crate::chrs::CharacterType;
use crate::cs;
use crate::custom_string::CustomString;
use crate::game::chain::Chain;
use crate::game::input::ChooseCardArgsP;
use crate::game::state::chr_info::CharacterInfo;
use crate::game::state::GameState;
use crate::game::GameCallbacks;
use crate::group::Group;
use crate::stats::StatType;
use crate::terminate;
use std::collections::BTreeSet;

acts! {
    // /*
    ПустаяКарта {
        name: cs!["ПУСТАЯ КАРТА"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использована"]),
            Point(cs!["выбери активку в руке. эта карта повторит эффект выбранной"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|game, args| {
                let owner_id = game.state().find_owner_act(args.act_id);
                let Some(imitated_act_id) = game.choose_act_in_hand(ChooseCardArgsP {
                    prompt: &cs![Active(ПустаяКарта), ": чей эффект повторить?"],
                    player_id: owner_id,
                    is_cancellable: true,
                    p: &|game_state, act_id| act_id != args.act_id && game_state.is_usable_in_any_way(act_id),
                }) else { terminate!() };

                todo!("повторить эффект {:?}", imitated_act_id)
            }),

            ..Default::default()
        },
    }

    Баян {
        name: cs!["БАЯН"],
        groups: [Group::СделаноМаксимом, Group::Дизморалит],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""], cs![Damage, " -= 3"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ЖёлтаяИскра {
        name: cs!["ЖЁЛТАЯ ИСКРА"],
        groups: [Group::СделаноЛёней, Group::Undertale],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs![Vitality, " = ", Physique]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let phy = game.state().chr(args.target_id).stats.phy.0.into_value();
                game.force_set_stat(args.target_id, StatType::Vitality, phy);

                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ТетрадьСмерти {
        name: cs!["ТЕТРАДЬ СМЕРТИ"],
        groups: [Group::СделаноКостей, Group::DeathNote],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs!["мгновенно убивает его"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let _ = game.die(args.target_id);
                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ОБратка {
        name: cs!["О,БРАТКА"],
        groups: [Group::СделаноЛёшей],

        description: cs![
            Condition(cs!["использована на противника, единственного на поле"]),
            Point(cs!["персонаж противника становится твоим и выставляется от тебя"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let owner_id = game.state().try_find_owner_act(args.act_id);
                let target_owner_id = game.state().try_find_owner_chr(args.target_id);

                if owner_id == target_owner_id {
                    terminate!()
                }

                todo!()
            }),

            ..Default::default()
        },
    }

    Коммунизм {
        name: cs!["КОММУНИЗМ"],
        groups: [Group::СделаноКостей, Group::ОбщественныйСтрой],

        description: cs![
            Condition(cs!["использован в качестве своего хода"]),
            Point(cs!["каждый игрок передаёт свою колоду следующему по направлению ходов"]),
            Point(cs!["эта карта уничтожается"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    Монархия {
        name: cs!["МОНАРХИЯ"],
        groups: [Group::СделаноЛёней, Group::ОбщественныйСтрой],

        description: cs![
            Condition(cs!["использована в ответ на ", Коммунизм]),
            Point(cs!["отменяет его эффект"]),
            Point(cs!["эта карта уничтожается"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    УтешительныйПриз {
        name: cs!["УТЕШИТЕЛЬНЫЙ ПРИЗ"],
        groups: [Group::СделаноЛёней, Group::TBoI, Group::Моралит],

        description: cs![
            Epitaph(cs![
                "толстой писал про эту медаль так:\n",
                "\"всевеликая всероссийская посеребрённая золотистая с платиновым отблеском заточенная\n",
                "медаль победителя всевеликого всероссийского этапа всевеликой всероссийской олимпиады\n",
                "всевеликих всероссийских школьников по всевеликому всеросскийскому животворящему программированию\""]),
            __,
            Condition(cs!["использован на персонажа"]),
            Point(cs!["статы, равные минимальному += 1"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|_game, _args| {
                todo!()
            }),

            ..Default::default()
        }
    }

    НеутешительныйПриз {
        name: cs!["НЕУТЕШИТЕЛЬНЫЙ ПРИЗ"],
        groups: [Group::СделаноМаксимом, Group::Дизморалит],

        // арт — уголёк

        description: cs![
            Epitaph(cs![
                "максим писал про эту медаль так:\n",
                "\"пепега какая-то\""]),
            __,
            Condition(cs!["использован на персонажа"]),
            Point(cs!["статы, равные максимальному -= 1"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|_game, _args| {
                todo!()
            }),

            ..Default::default()
        }
    }

    Биология {
        name: cs!["НЕДОСЫП"],
        groups: [Group::СделаноЛёней, Group::Реальность, Group::Дизморалит],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            Point(cs![Vitality, " & ", Intellect, " -= 4"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Vitality, -4);
                game.stat_add(args.target_id, StatType::Intellect, -4);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    СатокинаБита {
        name: cs!["САТОКИНА БИТА"],
        groups: [Group::СделаноКостей, Group::Higurashi],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs![Damage, " += 3"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Охаги {
        name: cs!["ОХАГИ"],
        groups: [Group::СделаноКостей, Group::Higurashi],

        description: cs![
            Condition(cs!["использованы на персонажа с ", Intellect, " ", LE, " 3"]),
            Point(cs!["наносят 1 ", Damage]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let chr_int = game.state().chr(args.target_id).stats.int.into_value();
                if !(chr_int <= 3) {
                    terminate!();
                }

                let _ = game.hurt(args.target_id, 1);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Тупость {
        name: cs!["ТУПОСТЬ"],
        groups: [Group::СделаноЛёней, Group::Моралит],

        description: cs![
            Condition(cs!["использована в ответ на ", Дизморалит, "-активку"]),
            Point(cs!["отменяет её эффект"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    Зеркало {
        name: cs!["ЗЕРКАЛО"],
        groups: [Group::СделаноЛёшей, Group::Реальность],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs!["копирует выбранную способность противника"]),
        ],

        // TODO
        // (нужна какая-то пометка, какие способности возможно копировать)
    }

    Хривна {
        name: cs!["ХРИВНА"],
        groups: [Group::СделаноКостей, Group::Реальность],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs![Intellect, " -= 1"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Intellect, -1);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    CuOH2 {
        name: cs!["CU(OH)₂"],
        groups: [Group::СделаноЛёней, Group::Химия],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            Point(cs![Vitality, " /= 2"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|_game, _args| {
                todo!()
            }),

            ..Default::default()
        }
    }

    МегаовощнойКейти {
        name: cs!["МЕГАОВОЩНОЙ КЕЙТИ"],
        groups: [Group::СделаноКостей, Group::Higurashi],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            Point(cs![Intellect, " = 0"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|_game, _args| {
                todo!("{} = 0", cs![Intellect])
            }),

            ..Default::default()
        }
    }

    Ластик {
        name: cs!["ЛАСТИК"],
        groups: [Group::СделаноЛёней, Group::Реальность],

        description: cs![
            Condition(cs!["использовано в качестве хода"]),
            Point(cs!["уничтожает все карты в бите и по одной выбранной каждым игроком у себя в руке"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    МойРотРазворот {
        name: cs!["МОЙ РОТ РАЗВОРОТ"],
        groups: [Group::СделаноЛёней, Group::Мемы],

        description: cs![
            Condition(cs!["использовано в начале своего хода"]),
            Point(cs!["меняет направление ходов на противоположное"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    Чёрт480 {
        name: cs!["ЧЁРТ 480"],
        groups: [Group::СделаноЛёней, Group::Скрытая, Group::ПепежноеСущество, Group::ЦитатыКости],

        description: cs![
            Condition(cs!["использовано в битве"]),
            Point(cs!["следующая активка, использованная противником, не сработает"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    ПионерУжеВКоммунизме {
        name: cs!["\"ЛЕЖИТ ПИОНЕР БЕЗ ПРИЗНАКОВ ЖИЗНИ, ЕМУ ХОРОШО, ОН УЖЕ В КОММУНИЗМЕ\""],
        groups: [Group::СделаноКостей, Group::Цитаты],

        description: cs![
            Condition(cs!["использовано на карту в руке"]),
            Point(cs!["отдай её следующему по направлению ходов игроку"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|_game, _args| {
                todo!();
            }),

            ..Default::default()
        }
    }

    Козерог {
        name: cs!["КОЗЕРОГ"],
        groups: [Group::СделаноЛёней, Group::TBoI, Group::Зодиак],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            NamedPoint(cs!["\"ALL STATS UP\""], cs![Physique, " & ", Vitality, " & ", Defence, " & ", Damage, " & ", Intellect, " += 2"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Physique, 2);
                game.stat_add(args.target_id, StatType::Vitality, 2);
                game.stat_add(args.target_id, StatType::Defence, 2);
                game.stat_add(args.target_id, StatType::Damage, 2);
                game.stat_add(args.target_id, StatType::Intellect, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    ЛезвиеНожа {
        name: cs!["ЛЕЗВИЕ НОЖА"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs![Damage, " += 1"]),
            Point(cs!["если ранее была использована ", РучкаНожа, ", получи ", Нож]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Physique, 1);

                #[allow(unreachable_code)]
                #[allow(clippy::diverging_sub_expression)]
                if todo!("ранее была использована РУЧКА НОЖА") {
                    let owner_id = game.state().find_owner_act(args.act_id);

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
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs![Physique, " += 1"]),
            Point(cs!["если ранее было использовано ", ЛезвиеНожа, ", получи ", Нож]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Physique, 1);

                #[allow(unreachable_code)]
                #[allow(clippy::diverging_sub_expression)]
                if todo!("ранее было использовано ЛЕЗВИЕ НОЖА") {
                    let owner_id = game.state().find_owner_act(args.act_id);

                    let drawn_chr_id = game.state_mut().chrs.add(CharacterInfo::new(CharacterType::Нож));
                    game.state_mut().chrs.add_to_player(drawn_chr_id, owner_id);
                }

                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    Берн {
        name: cs!["БЕРН"],
        groups: [Group::СделаноМаксимом, Group::Umineko],

        description: cs![
            Point(cs!["для ", ГВ, " ", Physique, " = 3"]),
            __,
            Condition(cs!["использована на противника, единственного на поле"]),
            Point(cs!["противник обязан поменять персонажа"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let target_owner_id = game.state().find_owner_chr(args.target_id);

                let Some(replacing_chr_id) = game.choose_chr_in_hand(ChooseCardArgsP {
                    prompt: &cs![Active(Берн), ": на кого поменять?"],
                    player_id: target_owner_id,
                    is_cancellable: true,
                    p: &GameState::is_placeable
                }) else { terminate!() };

                game.replace(args.target_id, replacing_chr_id);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Разум {
        name: cs!["РАЗУМ"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"I KNOW ALL\""], cs![Intellect, " += 3"]),
            Point(cs!["уже были использованы ", Тело, " и ", Душа, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Intellect, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Тело {
        name: cs!["ТЕЛО"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            NamedPoint(cs!["\"I FEEL ALL\""], cs![Physique, " & ", Vitality, " += 3"]),
            Point(cs!["уже были использованы ", Разум, " и ", Душа, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Physique, 3);
                game.stat_add(args.target_id, StatType::Vitality, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Душа {
        name: cs!["ДУША"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            NamedPoint(cs!["\"I AM ALL\""], cs![Defence, " += 3"]),
            Point(cs!["уже были использованы ", Разум, " и ", Тело, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Defence, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Godhead {
        name: cs!["GODHEAD"],
        groups: [Group::СделаноЛёней, Group::TBoI],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"GOD TEARS\""], cs![Damage, " += 3"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 3);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }
    // */
}
