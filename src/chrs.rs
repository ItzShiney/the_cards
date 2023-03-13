use crate::acts::ActiveType;
use crate::chrs;
use crate::cs;
use crate::dmg;
use crate::game::chain::Chain;
use crate::game::input::ChooseCardArgs;
use crate::game::input::PromptArgs;
use crate::game::GameCallbacks;
use crate::int;
use crate::phy;
use crate::stats::Stat0;
use crate::stats::StatType;
use crate::stats::Stats;
use crate::terminate;
use std::iter::repeat_with;

mod _macro;

chrs! {
    // /*
    БанкаСВареньем {
        name: cs!["БАНКА С ВАРЕНЬЕМ"],
        groups: [D, ByЛёня, Реальность],

        // 1/3/-0
        stats: Stats::new(
            phy!(1), // легко разбивается
            dmg!(2), // бьёт осколками
            int!(0),
        ),

        description: cs![
            Point(cs!["не атакует, если ", Intellect, " противника ", GE, " 3"])
        ],

        abilities: GameCallbacks {
            attack_map: Some(|game, args| {
                if game.state().chr(args.attacker_id).stats.int.0.into_value() >= 3 {
                    terminate!()
                } else {
                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    ДухТвоейКвартиры {
        name: cs!["ДУХ ТВОЕЙ КВАРТИРЫ"],
        groups: [B, ByКостя, Женщина],

        // 4/3/-4
        stats: Stats::new(
            phy!(8),
            dmg!(5),
            int!(1),
        ),

        // TODO
        description: cs![
            Epitaph(cs!["\"твоё личное бревно\""]),
            __,
            Condition(cs!["пока у владельца ", LE, " 2 персонажей"]),
            Point(cs![Damage, " больше на 2"]),
        ],
    }

    Планя {
        name: cs!["ПЛАНЯ"],
        groups: [C, ByКостя, Женщина, WePlanet],

        // 3/3/-4
        stats: Stats::new(
            phy!(4),
            dmg!(4),
            int!(2),
        ),

        // TODO
        description: cs![
            Condition(cs!["выставлена"]),
            NamedPoint(cs!["КРИНЖ И ПЕНИЕ"], cs![Intellect, " случайного персонажа в колоде противника -= 1"]),
            __,
            Condition(cs!["пока на поле"]),
            NamedPoint(cs!["МАКСИМАЛЬНАЯ СПЛЮЩЕННОСТЬ"], cs![Intellect, " всех персонажей на поле меньше на 4"]),
            __,
            Condition(cs!["персонаж из биты вернулся к владельцу"]),
            NamedPoint(cs!["\"ВЕРНИ САНКИ\""], cs![Physique, " всех персонажей в руке += 2"]),
        ],
    }

    Delirium {
        name: cs!["DELIRIUM"],
        groups: [C, ByМаксим, TBoI, Иллюзия],

        // ?/?/0
        stats: Stats::new(
            phy!(5?),
            dmg!(5?),
            int!(0), // представляет собой безумие
        ),

        description: cs![
            Condition(cs!["выставлен"]),
            Point(cs!["выбери персонажа в руке. ", Vitality, " = его ", Vitality, ", ", Damage, " = его ", Damage])
        ],

        abilities: GameCallbacks {
            post_place: Some(|game, args| {
                let self_id = args.chr_id;
                let owner_id = game.state().find_owner_chr(self_id);
                let Some(copied_chr_id) = game.choose_chr_in_hand_any(ChooseCardArgs {
                    prompt: PromptArgs {
                        str: cs![Character(Delirium), ": чьи ", Vitality, " и ", Damage, " скопировать?"],
                        is_cancellable: true,
                        autochoose_single_option: false,
                    },
                    player_id: owner_id,
                }) else { return };

                let stats = &game.state().chr(copied_chr_id).stats;
                let phy = stats.phy.0.into_value();
                let dmg = stats.dmg.0.into_value();

                game.force_set_phy_vit(self_id, phy);
                game.force_set_stat(self_id, StatType::Damage, dmg);
            }),

            ..Default::default()
        },
    }

    Беатриче {
        name: cs!["БЕАТРИЧЕ"],
        groups: [B, ByМаксим, Женщина, Umineko, Иллюзия],

        // 1/4/-3
        stats: Stats::new(
            phy!(5),
            dmg!(8),
            int!(7),
        ),

        description: cs![
            Condition(cs!["умерла"]),
            Point(cs!["с шансом 1/4 возвращается в руку"])
        ],

        abilities: GameCallbacks {
            die: Some(|game, args| {
                if game.random_bool(1./4.) {
                    terminate!()
                } else {
                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    Ненети {
        name: cs!["Н\u{0301}ЕНЕТИ"],
        groups: [C, ByЛёня, Женщина, NewGame],

        // 5/1/-1
        stats: Stats::new(
            phy!(7),
            dmg!(1),
            int!(4),
        ),
    }

    Коса {
        name: cs!["КОСА"],
        groups: [D, ByКостя, Женщина, Реальность],

        // 2/3/-1
        stats: Stats::new(
            phy!(3),
            dmg!(0),
            int!(8),
        ),
    }

    Мирослав {
        name: cs!["МИРОСЛАВ"],
        groups: [D, ByЛёня, Мужчина, Реальность],

        // 2/2/-4
        stats: Stats::new(
            phy!(3),
            dmg!(4),
            int!(0),
        ),
    }

    МаксимовБаянЖивотворящий {
        name: cs!["МАКСИМОВ БАЯН ЖИВОТВОРЯЩИЙ"],
        groups: [C, ByЛёня, Животворит],

        // 4/1/-0
        stats: Stats::new(
            phy!(6),
            dmg!(3),
            int!(8),
        ),
    }

    Рей {
        name: cs!["РЕЙ"],
        groups: [D, ByКостя, Мужчина],

        // 1/3/-2
        stats: Stats::new(
            phy!(2),
            dmg!(5),
            int!(6),
        ),
    }

    Тимми {
        name: cs!["ТИММИ"],
        groups: [D, ByКостя, Мужчина, SouthPark],

        // 1/0/-5
        stats: Stats::new(
            phy!(1),
            dmg!(0),
            int!(0),
        ),

        description: cs![
            Epitaph(cs!["\"тимми тимми тимми\""])
        ],
    }

    НостальгирующийКритик {
        name: cs!["НОСТАЛЬГИРУЮЩИЙ КРИТИК"],
        groups: [B, ByКостя, Мужчина],

        // 4/3/-2
        stats: Stats::new(
            phy!(7),
            dmg!(6),
            int!(6),
        ),

        // TODO
        description: cs![
            Condition(cs!["пока ", Intellect, " противника ", LE, " 3"]),
            Point(cs![Vitality, " этой карты на 1 меньше, ", Damage, " на 2 больше"]),
        ],

        abilities: GameCallbacks {
            stat_map: Some(|game, mut args| {
                let owner_id = game.state().find_owner_chr(args.chr_id);
                let Some(enemy_id) = game.state().try_enemy_chr_id(owner_id) else { return Chain::Continue(args) };
                let enemy_int = game.state().chr(enemy_id).stats.int.0.into_value();

                if enemy_int <= 3 {
                    match args.stat_type {
                        StatType::Vitality => args.val -= 1,
                        StatType::Damage => args.val += 2,
                        _ => {}
                    }
                }

                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Марио {
        name: cs!["МАРИО"],
        groups: [C, ByЛёня, Мужчина],

        // 2/2/-3
        stats: Stats::new(
            phy!(5),
            dmg!(5),
            int!(6),
        ),

        description: cs![
            Activatable,
            Condition(cs!["битва"]),
            NamedPoint(cs!["ПРЫЖОК НА ЛИЦО"], cs![Vitality, " противника /= 2"]),
        ],
    }

    Рена {
        name: cs!["РЕНА"],
        groups: [B, ByКостя, Женщина, Higurashi],

        // 2/3/-3
        stats: Stats::new(
            phy!(4),
            dmg!(7),
            int!(6),
        ),
    }

    Борат {
        name: cs!["БОРАТ"],
        groups: [C, ByКостя, Мужчина, Мемы],

        // 2/2/-4
        stats: Stats::new(
            phy!(4),
            dmg!(3),
            int!(1),
        ),

        description: cs![
            Condition(cs!["выставлен"]),
            NamedPoint(cs!["\"Я РЕПОРТЁР ИЗ КАЗАХСТАНА\""], cs!["возьми активку из стопки добора. если возможно, используй на этого персонажа, иначе положи обратно"]),
        ],

        abilities: GameCallbacks {
            post_place: Some(
                |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().find_owner_chr(self_id);

                    let Some(gained_act_id) = game.state_mut().acts.pick(owner_id) else { return };
                    if game.use_on_chr(gained_act_id, self_id).is_err() {
                        game.state_mut().acts.add_to_drawpile(gained_act_id);
                    }
                }
            ),

            ..Default::default()
        },
    }

    ЧёрныйКубик {
        name: cs!["ЧЁРНЫЙ КУБИК"],
        groups: [D, ByМаксим],

        // 3/1/-3
        stats: Stats::new(
            phy!(3),
            dmg!(1),
            int!(5),
        ),
    }

    Нож {
        name: cs!["НОЖ"],
        groups: [D, ByЛёня, TBoI, Нераздаваемая],

        // 2/?/-0
        stats: Stats::new(
            phy!(3),
            dmg!(5?),
            int!(1),
        ),

        description: cs![
            Condition(cs!["выставлен"]),
            Point(cs![Damage, " = ", Sum { times: cs!["9"], body: cs![Random(cs!["0"]..=cs!["1"])] }]),
        ],

        abilities: GameCallbacks {
            post_place: Some(
                |game, args| {
                    let value = repeat_with(|| { game.random(0, 1) }).take(9).sum();

                    let self_id = args.chr_id;
                    game.force_set_stat(self_id, StatType::Damage, value);
                }
            ),

            ..Default::default()
        },
    }

    ГлазКтулху {
        name: cs!["ГЛАЗ КТУЛХУ"],
        groups: [B, ByМаксим, Terraria],

        // 4/3/-3
        stats: Stats::new(
            phy!(8),
            dmg!(6),
            int!(2),
        ),

        description: cs![
            NamedPoint(cs!["\"ТАРАНИТ... ИНОГДА\""], cs!["с шансом 1/2 наносит на 1 ", Damage, " больше"]),
        ],

        abilities: GameCallbacks {
            attack_map: Some(|game, mut args| {
                if game.random_bool(1./2.) {
                    args.dmg += 1;
                }

                Chain::Continue(args)
            }),

            ..Default::default()
        }
    }

    Магдалина {
        name: cs!["МАГДАЛИНА"],
        groups: [C, ByЛёня, Женщина, TBoI],

        // 4/1/-2
        stats: Stats::new(
            phy!(7),
            dmg!(2),
            int!(6), // TODO брать у CharacterType::Айзек
        ),

        description: cs![
            Activatable,
            NamedPoint(cs!["НЯМ СЕРДЦЕ"], cs![Vitality, " += 2"]),
        ],
    }

    Рика {
        name: cs!["РИКА"],
        groups: [D, ByКостя, Женщина, Higurashi],

        // 1/1/-1
        stats: Stats::new(
            phy!(2),
            dmg!(2),
            int!(6),
        ),
    }

    Питон {
        name: cs!["ПИТОН"],
        groups: [B, ByЛёня, ЯзыкиПрограммирования],

        // 2/3/-0
        stats: Stats::new(
            phy!(5), // народная любовь
            dmg!(9), // больно от того, насколько он плох местами
            int!(3),
        ),

        // TODO
        // • удары дизморалят
    }

    Сатока {
        name: cs!["САТОКА"],
        groups: [D, ByЛёня, Женщина, Higurashi],

        // 3/2/-4
        stats: Stats::new(
            phy!(5), // терпит много лещей
            dmg!(3),
            int!(7), // ловушками перебивает спецотряд
        ),
    }

    Робеспьер {
        name: cs!["РОБЕСПЬЕР"],
        groups: [C, ByКостя, Мужчина, Реальность],

        // 2/5/-3
        stats: Stats::new(
            phy!(5),
            dmg!(5),
            int!(5),
        ),

        description: cs![
            Epitaph(cs!["\"vive la révolution\""]),
        ],
    }

    ГВ {
        name: cs!["ГВ"],
        groups: [C, ByМаксим, Мужчина, Женщина, Umineko],

        // 0/5/-3
        stats: Stats::new(
            phy!(0?),
            dmg!(7),
            int!(7),
        ),

        description: cs![
            // TODO
            Activatable,
            Point(cs!["этот персонаж превращается в выбранного из трёх случайных ", Umineko, "-персонажей"]),
            __,
            Condition(cs!["выставлен"]),
            Point(cs![Physique, " = ", SumAll { body: cs![Physique, " всех ", Иллюзия, " в руке"] }]),
            Point(cs!["считает ", Берн, " за персонажа с ", Physique, " 3"]),
        ],

        abilities: GameCallbacks {
            post_place: Some(
                |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().find_owner_chr(self_id);

                    let phy = {
                        let chrs_sum = game.state().chrs.hand(owner_id).iter().copied().filter_map(|chr_id| {
                            let chr = game.state().chr(chr_id);
                            if chr.type_.groups().contains(&Иллюзия) {
                                Some(chr.stats.phy.0.into_value())
                            } else {
                                None
                            }
                        }).sum::<Stat0>();

                        let acts_sum = game.state().acts.hand(owner_id).iter().copied().filter_map(|act_id| {
                            let act = game.state().act(act_id);
                            match act.type_ {
                                ActiveType::Берн => Some(3),
                                _ => None,
                            }
                        }).sum::<Stat0>();

                        chrs_sum + acts_sum
                    };

                    game.force_set_phy_vit(self_id, phy);
                }
            ),

            ..Default::default()
        },
    }
    // */
}
