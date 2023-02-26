use crate::cs;
#[allow(unused)]
use crate::{
    custom_string::CustomString, dmg, game_state::ability::character_ability::CharacterAbility,
    game_state::ability::character_trigger::CharacterTrigger, game_state::group::Group,
    gendered::RuGender, int, phy, stats::Stats,
};

use std::collections::BTreeSet;

macro_rules! chrs {
    (
        $(
            $CardName:ident {
                name: $name:expr,
                ru_gender: $ru_gender:expr,
                groups: $groups:tt,

                $(epitaph: $epitaph:expr,)?

                stats: $stats:expr,

                $(abilities: $abilities:tt,)?
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

            pub fn abilities(self) -> &'static Vec<CharacterAbility> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Vec<CharacterAbility> = Vec::from(
                            (
                                $($abilities,)?
                                {
                                    let x: [CharacterAbility; 0] = [];
                                    x
                                },
                            ).0
                        );
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

        stats: Stats::new(
            phy!(1),
            dmg!(2),
            int!(0),
        ),

        // TODO:
        // пока INT противника > 2 ⟹
        // • не атакует
    }

    ДухТвоейКвартиры {
        name: cs!["ДУХ ТВОЕЙ КВАРТИРЫ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Female],

        epitaph: cs!["\"твоё личное бревно\""],

        stats: Stats::new(
            phy!(7),
            dmg!(5),
            int!(2),
        ),

        // TODO:
        // пока персонажей у владельца <= 2 ⟹
        // • DMG больше на 2
    }

    Планя {
        name: cs!["ПЛАНЯ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::WePlanet],

        stats: Stats::new(
            phy!(5),
            dmg!(5),
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
        // • cs!["ВЕРНИ САНКИ"]: PHY всех персонажей в руке += 2
    }

    Delirium {
        name: cs!["DELIRIUM"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::TBoI, Group::Illusion],

        stats: Stats::new(
            phy!(?),
            dmg!(?),
            int!(0),
        ),

        abilities: [
            CharacterAbility {
                name: None,

                trigger: CharacterTrigger::Placed,
                conditions: vec![],

                description: cs!["выбери персонажа в руке. {vit} = его {vit}, {dmg} = его {dmg}"].into(),

                // TODO: заменить код с state_mut на set_phy_vit
                callback: |game, self_id, _went_trigger| {
                    let owner_id = game.state().chrs.find_owner(self_id).unwrap();
                    let copied_chr_id = game.choose_hand_chr(owner_id);

                    let stats = &game.state().chr(copied_chr_id).stats;
                    let copied_phy = stats.phy;
                    let copied_vit = stats.vit;

                    let self_ = game.state_mut().chr_mut(self_id);
                    self_.stats.phy = copied_phy;
                    self_.stats.vit = copied_vit;
                }
            }
        ],
    }

    Беатриче {
        name: cs!["БЕАТРИЧЕ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByMaxvog, Group::Female, Group::Umineko, Group::Illusion],

        stats: Stats::new(
            phy!(3),
            dmg!(8),
            int!(1),
        ),

        // TODO:
        // умерла ⟹
        // • с шансом 1/4 вернётся в руку
    }

    Ненети {
        name: cs!["Н\u{0301}ЕНЕТИ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Female, Group::NewGame],

        stats: Stats::new(
            phy!(5),
            dmg!(2),
            int!(2),
        ),
    }

    Коса {
        name: cs!["КОСА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::Reality],

        stats: Stats::new(
            phy!(3),
            dmg!(4),
            int!(8),
        ),
    }

    Мирослав {
        name: cs!["МИРОСЛАВ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Male, Group::Reality],

        stats: Stats::new(
            phy!(4),
            dmg!(4),
            int!(1),
        ),
    }

    МаксимовБаянЖивотворящий {
        name: cs!["МАКСИМОВ БАЯН ЖИВОТВОРЯЩИЙ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Lifemaking],

        stats: Stats::new(
            phy!(6),
            dmg!(4),
            int!(0),
        ),
    }

    Рей {
        name: cs!["РЕЙ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male],

        stats: Stats::new(
            phy!(1),
            dmg!(5),
            int!(6),
        ),
    }

    Тимми {
        name: cs!["ТИММИ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male, Group::SouthPark],

        epitaph: cs!["\"тимми тимми тимми\""],

        stats: Stats::new(
            phy!(2),
            dmg!(0),
            int!(0),
        ),
    }

    НостальгирующийКритик {
        name: cs!["НОСТАЛЬГИРУЮЩИЙ КРИТИК"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male],

        stats: Stats::new(
            phy!(6),
            dmg!(5),
            int!(5),
        ),

        // TODO:
        // пока INT противника <= 3 ⟹
        // • VIT этой карты на 1 меньше, DMG на 2 больше
    }

    Марио {
        name: cs!["МАРИО"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Male],

        stats: Stats::new(
            phy!(4),
            dmg!(5),
            int!(3),
        ),

        // TODO:
        // активируемая способность & битва ⟹
        // • ПРЫЖОК НА ЛИЦО: VIT противника /= 2
    }

    Рена {
        name: cs!["РЕНА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::Female, Group::Higurashi],

        stats: Stats::new(
            phy!(3),
            dmg!(5),
            int!(4),
        ),
    }

    Борат {
        name: cs!["БОРАТ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::Male, Group::Memes],

        stats: Stats::new(
            phy!(4),
            dmg!(3),
            int!(1),
        ),

        abilities: [
            CharacterAbility {
                name: None,

                trigger: CharacterTrigger::Placed,
                conditions: vec![],

                description: cs!["возьми активку из стопки добора. если возможно, используй на этого персонажа, иначе положи обратно"].into(),

                callback: |_game, _self_id, _went_trigger| {
                    todo!()
                }
            }
        ],
    }

    ЧёрныйКубик {
        name: cs!["ЧЁРНЫЙ КУБИК"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog],

        stats: Stats::new(
            phy!(5),
            dmg!(2),
            int!(3),
        ),
    }

    Нож {
        name: cs!["НОЖ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::TBoI, Group::Undrawable],

        stats: Stats::new(
            phy!(4),
            dmg!(?),
            int!(1),
        ),

        abilities: [
            CharacterAbility {
                name: None,

                trigger: CharacterTrigger::Placed,
                conditions: vec![],

                description: cs![Damage() " = " Sum { times: cs!["9"], body: cs![Random(cs!["0"]..=cs!["1"])] }].into(),

                callback: |_game, _self_id, _went_trigger| {
                    todo!()
                }
            }
        ],
    }

    ГлазКтулху {
        name: cs!["ГЛАЗ КТУЛХУ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::Terraria],

        stats: Stats::new(
            phy!(8),
            dmg!(6),
            int!(2),
        ),

        // TODO:
        // атакует ⟹
        // • "ТАРАНИТ... ИНОГДА": с шансом 50% наносит на 1 больше
    }
    // */

    Магдалина {
        name: cs!["МАГДАЛИНА"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByShiney, Group::Female, Group::TBoI],

        stats: Stats::new(
            phy!(7),
            dmg!(2),
            int!(4),
        ),

        // TODO:
        // активируемая способность ⟹
        // • НЯМ СЕРДЦЕ: VIT += 2
    }
}
