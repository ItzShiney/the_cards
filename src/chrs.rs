use crate::custom_string::CustomString;
use crate::def;
use crate::dmg;
use crate::game_state::ability::character_ability::CharacterAbility;
use crate::game_state::group::Group;
use crate::gendered::RuGender;
use crate::int;
use crate::phy;
use crate::stats::Stats;

use std::collections::BTreeSet;

macro_rules! chrs {
    (
        $(
            $CardName:ident {
                const NAME = $name:literal;
                const RU_GENDER = $ru_gender:expr;
                const GROUPS = $groups:tt;

                const STATS = $stats:expr;

                const ABILITIES = $abilities:tt;
            }
        )*
    ) => {paste::paste!{
        #[derive(Clone, Copy)]
        pub enum CharacterType {
            $($CardName,)*
        }

        impl CharacterType {
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
                        static ref [<$CardName:snake:upper>]: BTreeSet<Group> = BTreeSet::<Group>::from($groups);
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
                        static ref [<$CardName:snake:upper>]: Vec<CharacterAbility> = vec! $abilities;
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
    TestCharacter {
        const NAME = "ТЕСТОВЫЙ ПЕРС";
        const RU_GENDER = RuGender::Masculine;
        const GROUPS = [];

        const STATS = Stats::new(
            phy!(5),
            def!(2),
            dmg!(3),
            int!(0),
        );

        const ABILITIES = [];
    }

    JarOfJam {
        const NAME = "БАНКА С ВАРЕНЬЕМ";
        const RU_GENDER = RuGender::Feminine;
        const GROUPS = [Group::Reality];

        const STATS = Stats::new(
            phy!(5),
            def!(2),
            dmg!(3),
            int!(0),
        );

        const ABILITIES = [];
    }
}
