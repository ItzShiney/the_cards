#[allow(unused)]
use crate::{
    chrs::CharacterType, cs, custom_string::CustomString, game_state::group::Group,
    gendered::RuGender,
};
use crate::{
    described::Described,
    game_state::ability_description::AbilityDescription,
    host::{Chain, GameCallbacks},
    stats::Stat,
};

use std::collections::BTreeSet;

macro_rules! acts {
    (
        $(
            $CardName:ident {
                name: $name:expr,
                ru_gender: $ru_gender:expr,
                groups: $groups:tt,

                $(epitaph: $epitaph:expr,)?

                abilities: $abilities:expr,
            }
        )*
    ) => {paste::paste!{
        #[derive(Clone, Copy)]
        pub enum ActiveType {
            $($CardName,)*
        }

        impl ActiveType {
            pub fn all() -> Vec<Self> {
                vec![
                    $(Self::$CardName,)*
                ]
            }

            pub fn name(self) -> &'static $crate::custom_string::CustomString {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::custom_string::CustomString = $name;
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
                }
            }

            pub fn ru_gender(self) -> $crate::gendered::RuGender {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::gendered::RuGender = $ru_gender;
                    )*
                }

                match self {
                    $(Self::$CardName => *[<$CardName:snake:upper>],)*
                }
            }

            pub fn groups(self) -> &'static BTreeSet<Group> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: BTreeSet<Group> = BTreeSet::<Group>::from($groups);
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
                }
            }

            pub fn epitaph(self) -> &'static Option<CustomString> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Option<CustomString> = {
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

            pub fn abilities(self) -> &'static $crate::host::GameCallbacks {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::host::GameCallbacks = $abilities;
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
                }
            }
        }
    }};
}

acts! {
    ПустаяКарта {
        name: cs!["ПУСТАЯ КАРТА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: GameCallbacks {
            use_on_field: Some(
                Described { description: AbilityDescription {
                    name: None,
                    description: cs!["выбери активку в руке. эта карта повторит эффект выбранной"],
                },

                value: |game, args| {
                    let owner_id = game.state().acts.find_owner(args.act_id).unwrap();
                    let imitated_act_id = game.choose_hand_act(owner_id);

                    todo!("mimic {:?}", imitated_act_id)
                }
            }),

            ..Default::default()
        },
    }

    // /*
    Баян {
        name: cs!["БАЯН"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::Dismoral],

        abilities: GameCallbacks {
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: Some(cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""]),
                    description: cs![Damage, " -= 3"],
                },

                value: |game, args| {
                    game.modify_stat(args.target_id, Stat::Damage, 3);
                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    ЖёлтаяИскра {
        name: cs!["ЖЁЛТАЯ ИСКРА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Undertale],

        abilities: GameCallbacks {
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![Vitality, " = ", Physique],
                },

                value: |game, args| {
                    let phy = game.state().chr(args.target_id).stats.phy.0.into_value().unwrap();
                    game.set_stat(args.target_id, Stat::Vitality, phy);

                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    ТетрадьСмерти {
        name: cs!["ТЕТРАДЬ СМЕРТИ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::DeathNote],

        abilities: GameCallbacks {
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs!["мгновенно убивает его"],
                },

                value: |game, args| {
                    game.kill(args.target_id);
                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }

    Коммунизм {
        name: cs!["КОММУНИЗМ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::SocialOrder],

        abilities: GameCallbacks {
            // TODO: использовано в качестве своего хода =>
            use_on_field: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![
                        "каждый игрок передаёт свою колоду следующему по направлению ходов\n",
                        Bullet, "эта карта уничтожается\n",
                        Bullet, "ход заканчивается"
                    ],
                },

                value: |_game, _args| {
                    todo!()
                }
            }),

            ..Default::default()
        },
    }

    ОБратка {
        name: cs!["О,БРАТКА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByZoinX],

        abilities: GameCallbacks {
            // TODO: использовано на противника & твой персонаж не выставлен =>
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs!["персонаж противника становится твоим и выставляется от тебя"],
                },

                value: |game, args| {
                    let self_id = args.act_id;
                    let owner_id = game.state().acts.find_owner(self_id);

                    let target_id = args.target_id;
                    let target_owner_id = game.state().chrs.find_owner(target_id);

                    if owner_id == target_owner_id {
                        return Chain::Break(Err(()));
                    }

                    todo!()
                }
            }),

            ..Default::default()
        },
    }

    ЛезвиеНожа {
        name: cs!["ЛЕЗВИЕ НОЖА"],
        ru_gender: RuGender::Neuter,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: GameCallbacks {
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![ // FIXME
                        Damage, " += 1\n",
                        Bullet, " если ранее была использована ", РучкаНожа, ", получи ", Нож
                    ],
                },

                value: |_game, _args| {
                    todo!()
                }
            }),

            ..Default::default()
        },
    }

    РучкаНожа {
        name: cs!["РУЧКА НОЖА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: GameCallbacks {
            use_on_character: Some(Described {
                description: AbilityDescription {
                    name: None,
                    description: cs![ // FIXME
                        Physique, " += 1\n",
                        Bullet, " если ранее было использовано ", ЛезвиеНожа, ", получи ", Нож
                    ],
                },

                value: |game, args| {
                    let target_id = args.target_id;
                    game.modify_stat(target_id, Stat::Physique, 1);

                    Chain::Continue(args)
                }
            }),

            ..Default::default()
        },
    }
    // */
}
