use crate::{
    cs,
    custom_string::CustomString,
    described::Described,
    dmg,
    game_state::ability_description::AbilityDescription,
    game_state::group::Group,
    gendered::RuGender,
    host::{Chain, GameCallbacks},
    int, phy,
    stats::{Stat, Stats},
};

use std::{collections::BTreeSet, iter::repeat_with};

macro_rules! chrs {
    (
        $(
            $CardName:ident {
                name: $name:expr,
                ru_gender: $ru_gender:expr,
                groups: $groups:tt,

                $(epitaph: $epitaph:expr,)?

                stats: $stats:expr,

                $(abilities: $abilities:expr,)?
            }
        )*
    ) => {paste::paste!{
        #[derive(Clone, Copy)]
        pub enum CharacterType {
            $($CardName,)*
        }

        impl CharacterType {
            pub fn all() -> Vec<Self> {
                vec![
                    $(Self::$CardName,)*
                ]
            }

            pub fn name(self) -> &'static CustomString {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: CustomString = $name.into();
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn ru_gender(self) -> RuGender {
                match self {
                    $(Self::$CardName => $ru_gender,)*
                }
            }

            pub fn groups(self) -> &'static BTreeSet<Group> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: BTreeSet<Group> = BTreeSet::from($groups);
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn epitaph(self) -> &'static Option<CustomString> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Option<CustomString> =  {
                            let x = (
                                $($epitaph,)?
                                cs![],
                            ).0;
                            if x.slices.is_empty() {
                                None
                            } else {
                                Some(x)
                            }
                        };
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn stats(self) -> Stats {
                match self {
                    $(Self::$CardName => $stats,)*
                }
            }

            pub fn abilities(self) -> &'static $crate::host::GameCallbacks {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::host::GameCallbacks =
                            (
                                $($abilities,)?
                                $crate::host::GameCallbacks::default(),
                            ).0;
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }
        }
    }};
}

chrs! {
    // /*
    БанкаСВареньем {
        name: cs!["БАНКА С ВАРЕНЬЕМ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Reality],

        // 1/3/-0
        stats: Stats::new(
            phy!(1), // легко разбивается
            dmg!(2), // бьёт осколками
            int!(0),
        ),

        abilities: GameCallbacks {
            attack: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs!["не атакует, если ", Intellect, " противника ", GE, " 3"],
                },

                value: |game, args| {
                    let self_id = args.attacker_id;

                    if game.state().chr(self_id).stats.int.0.into_value() >= 3 {
                        return Chain::Break(());
                    }

                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    ДухТвоейКвартиры {
        name: cs!["ДУХ ТВОЕЙ КВАРТИРЫ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Female],

        epitaph: cs!["\"твоё личное бревно\""],

        // 4/3/-4
        stats: Stats::new(
            phy!(8),
            dmg!(5),
            int!(1),
        ),

        // TODO:
        // пока персонажей у владельца <= 2 ⟹
        // • DMG больше на 2
    }

    Планя {
        name: cs!["ПЛАНЯ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::WePlanet],

        // 3/3/-4
        stats: Stats::new(
            phy!(4),
            dmg!(4),
            int!(2),
        ),

        // TODO:
        // выставлена ⟹
        // • КРИНЖ И ПЕНИЕ: INT случайного персонажа в колоде противника -= 1
        //
        // пока на поле ⟹
        // • МАКСИМАЛЬНАЯ СПЛЮЩЕННОСТЬ: INT всех персонажей на поле меньше на 4
        //
        // персонаж из биты вернулся к владельцу ⟹
        // • "ВЕРНИ САНКИ": PHY всех персонажей в руке += 2
    }

    Delirium {
        name: cs!["DELIRIUM"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::TBoI, Group::Illusion],

        // ?/?/0
        stats: Stats::new(
            phy!(5?),
            dmg!(5?),
            int!(0), // представляет собой безумие
        ),

        abilities: GameCallbacks {
            post_place: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs!["выбери персонажа в руке. ", Vitality, " = его ", Vitality, ", ", Damage, " = его ", Damage],
                },

                value: |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().chrs.find_owner(self_id);
                    let copied_chr_id = game.choose_hand_chr(owner_id);
                    // println!("DELIRIUM копирует:\n{}", game.state().chr(copied_chr_id));

                    let stats = &game.state().chr(copied_chr_id).stats;
                    let phy = stats.phy.0.into_value();
                    let dmg = stats.dmg.0.into_value();

                    game.set_phy_vit(self_id, phy);
                    game.set_stat(self_id, Stat::Damage, dmg);
                }
            }),

            ..Default::default()
        },
    }

    Беатриче {
        name: cs!["БЕАТРИЧЕ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByMaxvog, Group::Female, Group::Umineko, Group::Illusion],

        // 1/4/-3
        stats: Stats::new(
            phy!(5),
            dmg!(8),
            int!(7),
        ),

        // TODO:
        // умерла ⟹
        // • с шансом 1/4 вернётся в руку
    }

    Ненети {
        name: cs!["Н\u{0301}ЕНЕТИ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Female, Group::NewGame],

        // 5/1/-1
        stats: Stats::new(
            phy!(7),
            dmg!(1),
            int!(4),
        ),
    }

    Коса {
        name: cs!["КОСА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::Reality],

        // 2/3/-1
        stats: Stats::new(
            phy!(3),
            dmg!(0),
            int!(8),
        ),
    }

    Мирослав {
        name: cs!["МИРОСЛАВ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Male, Group::Reality],

        // 2/2/-4
        stats: Stats::new(
            phy!(3),
            dmg!(4),
            int!(0),
        ),
    }

    МаксимовБаянЖивотворящий {
        name: cs!["МАКСИМОВ БАЯН ЖИВОТВОРЯЩИЙ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Lifemaking],

        // 4/1/-0
        stats: Stats::new(
            phy!(6),
            dmg!(3),
            int!(8),
        ),
    }

    Рей {
        name: cs!["РЕЙ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male],

        // 1/3/-2
        stats: Stats::new(
            phy!(2),
            dmg!(5),
            int!(6),
        ),
    }

    Тимми {
        name: cs!["ТИММИ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male, Group::SouthPark],

        epitaph: cs!["\"тимми тимми тимми\""],

        // 1/0/-5
        stats: Stats::new(
            phy!(1),
            dmg!(0),
            int!(0),
        ),
    }

    НостальгирующийКритик {
        name: cs!["НОСТАЛЬГИРУЮЩИЙ КРИТИК"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male],

        // 4/3/-2
        stats: Stats::new(
            phy!(7),
            dmg!(6),
            int!(6),
        ),

        // TODO:
        // пока INT противника <= 3 ⟹
        // • VIT этой карты на 1 меньше, DMG на 2 больше
    }

    Марио {
        name: cs!["МАРИО"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Male],

        // 2/2/-3
        stats: Stats::new(
            phy!(5),
            dmg!(5),
            int!(6),
        ),

        // TODO:
        // активируемая способность & битва ⟹
        // • ПРЫЖОК НА ЛИЦО: VIT противника /= 2
    }

    Рена {
        name: cs!["РЕНА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::Higurashi],

        // 2/3/-3
        stats: Stats::new(
            phy!(4),
            dmg!(7),
            int!(6),
        ),
    }

    Борат {
        name: cs!["БОРАТ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male, Group::Memes],

        // 2/2/-4
        stats: Stats::new(
            phy!(4),
            dmg!(3),
            int!(1),
        ),

        abilities: GameCallbacks {
            post_place: Some(Described {
                description: AbilityDescription {
                    name: Some(cs!["\"Я РЕПОРТЁР ИЗ КАЗАХСТАНА\""]),
                    description: cs!["возьми активку из стопки добора. если возможно, используй на этого персонажа, иначе положи обратно"],
                },

                value: |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().chrs.find_owner(self_id);

                    let Some(gained_act_id) = game.state_mut().acts.pick(owner_id) else { return };
                    if game.use_on_character(gained_act_id, self_id).is_err() {
                        game.state_mut().acts.add_to_drawpile(gained_act_id);
                    }
                }
            }),

            ..Default::default()
        },
    }

    ЧёрныйКубик {
        name: cs!["ЧЁРНЫЙ КУБИК"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog],

        // 3/1/-3
        stats: Stats::new(
            phy!(3),
            dmg!(1),
            int!(5),
        ),
    }

    Нож {
        name: cs!["НОЖ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::TBoI, Group::Undrawable],

        // 2/?/-0
        stats: Stats::new(
            phy!(3),
            dmg!(5?),
            int!(1),
        ),

        abilities: GameCallbacks {
            post_place: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![Damage, " = ", Sum { times: cs!["9"], body: cs![Random(cs!["0"]..=cs!["1"])] }],
                },

                value: |game, args| {
                    let value = repeat_with(|| { game.random(0, 1) }).take(9).sum();

                    let self_id = args.chr_id;
                    game.set_stat(self_id, Stat::Damage, value);
                }
            }),

            ..Default::default()
        },
    }

    ГлазКтулху {
        name: cs!["ГЛАЗ КТУЛХУ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::Terraria],

        // 4/3/-3
        stats: Stats::new(
            phy!(8),
            dmg!(6),
            int!(2),
        ),

        // TODO:
        // атакует ⟹
        // • "ТАРАНИТ... ИНОГДА": с шансом 50% наносит на 1 больше

        /*
        callbacks: {
            attack: |game, self_id, mut args| {
                if game.random_bool(0.5) {
                    args.damage += 1;
                }

                Chain::Continue(args)
            },

            ..default()
        }
        */
    }

    Магдалина {
        name: cs!["МАГДАЛИНА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Female, Group::TBoI],

        // 4/1/-2
        stats: Stats::new(
            phy!(7),
            dmg!(2),
            int!(6), // TODO: брать у CharacterType::Айзек
        ),

        // TODO:
        // активируемая способность ⟹
        // • НЯМ СЕРДЦЕ: VIT += 2
    }

    Рика {
        name: cs!["РИКА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::Higurashi],

        // 1/1/-1
        stats: Stats::new(
            phy!(2),
            dmg!(2),
            int!(6),
        ),
    }

    Питон {
        name: cs!["ПИТОН"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::ProgrammingLanguages],

        // 2/3/-0
        stats: Stats::new(
            phy!(5), // народная любовь
            dmg!(9), // больно от того, насколько он плох местами
            int!(3),
        ),

        // TODO:
        // • удары дизморалят
    }

    Сатока {
        name: cs!["САТОКА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Female, Group::Higurashi],

        // 3/2/-4
        stats: Stats::new(
            phy!(5), // терпит много лещей
            dmg!(3),
            int!(7), // ловушками перебивает спецотряд
        ),
    }

    Робеспьер {
        name: cs!["РОБЕСПЬЕР"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male, Group::Reality],

        epitaph: cs!["\"vive la révolution\""],

        // 2/5/-3
        stats: Stats::new(
            phy!(5),
            dmg!(5),
            int!(5),
        ),
    }
    // */

    ГВ {
        name: cs!["ГВ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::Male, Group::Female, Group::Umineko],

        // 0/5/-3
        stats: Stats::new(
            phy!(5?),
            dmg!(7),
            int!(7),
        ),

        // TODO:
        // активируемая способность:
        // • выбери [umineko]-персонажа и замени на него

        abilities: GameCallbacks {
            post_place: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![Physique, " = ", SumAll { body: cs![Physique, " всех ", Group(Illusion), " в руке"] }],
                },

                value: |game, args| {
                    let self_id = args.chr_id;
                    let owner_id = game.state().chrs.find_owner(self_id);

                    let phy = {
                        game.state().chrs.hand(owner_id).clone().into_iter().filter_map(|chr_id| {
                            let chr = game.state().chr(chr_id);
                            // TODO: проверять все группы, не только главные (groups())
                            if chr.type_.groups().contains(&Group::Illusion) {
                                Some(chr.stats.phy.0.into_value())
                            } else {
                                None
                            }
                        }).sum()
                    };

                    game.set_phy_vit(self_id, phy);
                }
            }),

            ..Default::default()
        },
    }
}
