mod _macro;

use crate::acts;
use crate::chrs::CharacterType;
use crate::cs;
use crate::game::chain::Chain;
use crate::game::input::ChooseCardArgsP;
use crate::game::input::PromptArgs;
use crate::game::state::chr_info::CharacterInfo;
use crate::game::GameCallbacks;
use crate::stats::StatType;
use crate::terminate;

acts! {
    // /*
    ПустаяКарта {
        name: cs!["ПУСТАЯ КАРТА"],
        groups: [B, ByЛёня, TBoI],

        description: cs![
            Condition(cs!["использована"]),
            Point(cs!["выбери активку в руке. эта карта повторит эффект выбранной"]),
        ],

        abilities: GameCallbacks {
            use_on_field: Some(|game, args| {
                let owner_id = game.state().find_owner_act(args.act_id);
                let Some(imitated_act_id) = game.choose_act_in_hand(ChooseCardArgsP {
                    prompt: PromptArgs {
                        str: cs![Active(ПустаяКарта), ": чей эффект повторить?"],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    player_id: owner_id,
                    p: &|game_state, act_id| act_id != args.act_id && game_state.is_usable_in_any_way(act_id),
                }) else { terminate!() };

                todo!("повторить эффект {:?}", imitated_act_id)
            }),

            ..Default::default()
        },
    }

    Баян {
        name: cs!["БАЯН"],
        groups: [D, ByМаксим, Дизморалит],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""], cs![Damage, " -= 1"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 1);
                Chain::Continue(args)
            }),

            ..Default::default()
        },
    }

    ЖёлтаяИскра {
        name: cs!["ЖЁЛТАЯ ИСКРА"],
        groups: [D, ByЛёня, Undertale],
        // D, потому что работает только после активок типа "наносит урон", а таких мало

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
        groups: [B, ByКостя, DeathNote],

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
        groups: [A, ByЛёша],

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
        groups: [S, ByКостя, ОбщественныйСтрой],

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
        groups: [D, ByЛёня, ОбщественныйСтрой],

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
        groups: [D, ByЛёня, TBoI, Моралит],

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
        groups: [D, ByМаксим, Дизморалит],

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
        groups: [D, ByЛёня, Реальность, Дизморалит],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            Point(cs![Vitality, " & ", Intellect, " -= 2"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Vitality, -2);
                game.stat_add(args.target_id, StatType::Intellect, -2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    СатокинаБита {
        name: cs!["САТОКИНА БИТА"],
        groups: [C, ByКостя, Higurashi],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            Point(cs![Damage, " += 2"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Охаги {
        name: cs!["ОХАГИ"],
        groups: [D, ByКостя, Higurashi],

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
        groups: [D, ByЛёня, Моралит],

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
        groups: [D, ByЛёша, Реальность],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            Point(cs!["копирует выбранную способность противника"]),
        ],

        // TODO
        // (нужна какая-то пометка, какие способности возможно копировать)
    }

    Хривна {
        name: cs!["ХРИВНА"],
        groups: [D, ByКостя, Реальность],

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
        groups: [C, ByЛёня, Химия],

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
        groups: [C, ByКостя, Higurashi],

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
        groups: [D, ByЛёня, Реальность],

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
        groups: [D, ByЛёня, Мемы],

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
        groups: [C, ByЛёня, Скрытая, ПепежноеСущество, ЦитатыКости],

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
        groups: [D, ByКостя, Цитаты],

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
        groups: [B, ByЛёня, TBoI, Зодиак],

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
        groups: [D, ByЛёня, TBoI],

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
        groups: [D, ByЛёня, TBoI],

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
        groups: [C, ByМаксим, Umineko],

        description: cs![
            Condition(cs!["использована на противника, единственного на поле"]),
            Point(cs!["противник обязан поменять персонажа"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                let target_owner_id = game.state().find_owner_chr(args.target_id);

                let Some(replacing_chr_id) = game.choose_chr_in_hand(ChooseCardArgsP {
                    prompt: PromptArgs {
                        str: cs![Active(Берн), ": на кого поменять?"],
                        is_cancellable: false,
                        autochoose_single_option: true,
                    },
                    player_id: target_owner_id,
                    p: &|game_state, chr_id| chr_id != args.target_id && game_state.is_placeable(chr_id)
                }) else { terminate!() };

                game.replace(args.target_id, replacing_chr_id);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Разум {
        name: cs!["РАЗУМ"],
        groups: [D, ByЛёня, TBoI],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"I KNOW ALL\""], cs![Intellect, " += 2"]),
            Point(cs!["уже были использованы ", Тело, " и ", Душа, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Intellect, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Тело {
        name: cs!["ТЕЛО"],
        groups: [C, ByЛёня, TBoI],

        description: cs![
            Condition(cs!["использовано на персонажа"]),
            NamedPoint(cs!["\"I FEEL ALL\""], cs![Physique, " & ", Vitality, " += 2"]),
            Point(cs!["уже были использованы ", Разум, " и ", Душа, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Physique, 2);
                game.stat_add(args.target_id, StatType::Vitality, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Душа {
        name: cs!["ДУША"],
        groups: [D, ByЛёня, TBoI],

        description: cs![
            Condition(cs!["использована на персонажа"]),
            NamedPoint(cs!["\"I AM ALL\""], cs![Defence, " += 2"]),
            Point(cs!["уже были использованы ", Разум, " и ", Тело, " ", Implies, " получи ", Godhead]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Defence, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Godhead {
        name: cs!["GODHEAD"],
        groups: [C, ByЛёня, TBoI],

        description: cs![
            Condition(cs!["использован на персонажа"]),
            NamedPoint(cs!["\"GOD TEARS\""], cs![Damage, " += 2"]),
        ],

        abilities: GameCallbacks {
            use_on_chr: Some(|game, args| {
                game.stat_add(args.target_id, StatType::Damage, 2);
                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }
    // */
}
