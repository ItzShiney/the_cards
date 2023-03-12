mod _macro;

use crate::acts::ActiveType;
use crate::chrs;
use crate::cs;
use crate::custom_string::CustomString;
use crate::dmg;
use crate::game::chain::Chain;
use crate::game::input::ChooseCardArgs;
use crate::game::GameCallbacks;
use crate::group::Group;
use crate::int;
use crate::phy;
use crate::stats::Stat0;
use crate::stats::StatType;
use crate::stats::Stats;
use crate::terminate;
use std::collections::BTreeSet;
use std::iter::repeat_with;

chrs! {
    // /*
    БанкаСВареньем {
        name: cs!["БАНКА С ВАРЕНЬЕМ"],
        groups: [Group::СделаноЛёней, Group::Реальность],

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
        groups: [Group::СделаноКостей, Group::Женщина],

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
        groups: [Group::СделаноКостей, Group::Женщина, Group::WePlanet],

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
        groups: [Group::СделаноМаксимом, Group::TBoI, Group::Иллюзия],

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
                let owner_id = game.state().chrs.find_owner(self_id);
                let Some(copied_chr_id) = game.choose_chr_in_hand_any(ChooseCardArgs {
                    player_id: owner_id,
                    is_cancellable: true,
                }) else { return };

                // println!("DELIRIUM копирует:\n{}", game.state().chr(copied_chr_id));

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
        groups: [Group::СделаноМаксимом, Group::Женщина, Group::Umineko, Group::Иллюзия],

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
        groups: [Group::СделаноЛёней, Group::Женщина, Group::NewGame],

        // 5/1/-1
        stats: Stats::new(
            phy!(7),
            dmg!(1),
            int!(4),
        ),
    }

    Коса {
        name: cs!["КОСА"],
        groups: [Group::СделаноКостей, Group::Женщина, Group::Реальность],

        // 2/3/-1
        stats: Stats::new(
            phy!(3),
            dmg!(0),
            int!(8),
        ),
    }

    Мирослав {
        name: cs!["МИРОСЛАВ"],
        groups: [Group::СделаноЛёней, Group::Мужчина, Group::Реальность],

        // 2/2/-4
        stats: Stats::new(
            phy!(3),
            dmg!(4),
            int!(0),
        ),
    }

    МаксимовБаянЖивотворящий {
        name: cs!["МАКСИМОВ БАЯН ЖИВОТВОРЯЩИЙ"],
        groups: [Group::СделаноЛёней, Group::Животворит],

        // 4/1/-0
        stats: Stats::new(
            phy!(6),
            dmg!(3),
            int!(8),
        ),
    }

    Рей {
        name: cs!["РЕЙ"],
        groups: [Group::СделаноКостей, Group::Мужчина],

        // 1/3/-2
        stats: Stats::new(
            phy!(2),
            dmg!(5),
            int!(6),
        ),
    }

    Тимми {
        name: cs!["ТИММИ"],
        groups: [Group::СделаноКостей, Group::Мужчина, Group::SouthPark],

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
        groups: [Group::СделаноКостей, Group::Мужчина],

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
                let Some(enemy_id) = game.state().enemy_chr_id(args.chr_id) else { return Chain::Continue(args) };
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
        groups: [Group::СделаноЛёней, Group::Мужчина],

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
        groups: [Group::СделаноКостей, Group::Женщина, Group::Higurashi],

        // 2/3/-3
        stats: Stats::new(
            phy!(4),
            dmg!(7),
            int!(6),
        ),
    }

    Борат {
        name: cs!["БОРАТ"],
        groups: [Group::СделаноКостей, Group::Мужчина, Group::Мемы],

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
                    let owner_id = game.state().chrs.find_owner(self_id);

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
        groups: [Group::СделаноМаксимом],

        // 3/1/-3
        stats: Stats::new(
            phy!(3),
            dmg!(1),
            int!(5),
        ),
    }

    Нож {
        name: cs!["НОЖ"],
        groups: [Group::СделаноЛёней, Group::TBoI, Group::Нераздаваемая],

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
        groups: [Group::СделаноМаксимом, Group::Terraria],

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
        groups: [Group::СделаноЛёней, Group::Женщина, Group::TBoI],

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
        groups: [Group::СделаноКостей, Group::Женщина, Group::Higurashi],

        // 1/1/-1
        stats: Stats::new(
            phy!(2),
            dmg!(2),
            int!(6),
        ),
    }

    Питон {
        name: cs!["ПИТОН"],
        groups: [Group::СделаноЛёней, Group::ЯзыкиПрограммирования],

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
        groups: [Group::СделаноЛёней, Group::Женщина, Group::Higurashi],

        // 3/2/-4
        stats: Stats::new(
            phy!(5), // терпит много лещей
            dmg!(3),
            int!(7), // ловушками перебивает спецотряд
        ),
    }

    Робеспьер {
        name: cs!["РОБЕСПЬЕР"],
        groups: [Group::СделаноКостей, Group::Мужчина, Group::Реальность],

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
        groups: [Group::СделаноМаксимом, Group::Мужчина, Group::Женщина, Group::Umineko],

        // 0/5/-3
        stats: Stats::new(
            phy!(0?),
            dmg!(7),
            int!(7),
        ),

        description: cs![
            // TODO
            Activatable,
            Point(cs!["выбери ", Umineko, "-персонажа в руке и замени этого на него"]),
            __,
            Condition(cs!["выставлен"]),
            Point(cs![Physique, " = ", SumAll { body: cs![Physique, " всех ", Иллюзия, " в руке"] }])
        ],

        abilities: GameCallbacks {
            post_place: Some(
                |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().chrs.find_owner(self_id);

                    let phy = {
                        let chrs_sum = game.state().chrs.hand(owner_id).iter().copied().filter_map(|chr_id| {
                            let chr = game.state().chr(chr_id);
                            if chr.type_.groups().contains(&Group::Иллюзия) {
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
