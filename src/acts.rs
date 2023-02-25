use crate::{
    custom_string::CustomString,
    game_state::ability::active_ability::ActiveAbility,
    game_state::ability::active_trigger::{ActiveTrigger, WentActiveTrigger},
    game_state::group::Group,
    gendered::RuGender,
};

use std::collections::BTreeSet;

macro_rules! acts {
    (
        $(
            $CardName:ident {
                name: $name:literal,
                ru_gender: $ru_gender:expr,
                groups: $groups:tt,

                $(epitaph: $epitaph:literal,)?

                abilities: $abilities:tt,
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

            pub fn name(self) -> &'static CustomString {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: CustomString = $name.into();
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
                }
            }

            pub fn ru_gender(self) -> RuGender {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: RuGender = $ru_gender;
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
                        static ref [<$CardName:snake:upper>]: Option<CustomString> = if concat!("", $($epitaph)?) != "" { Some(concat!("", $($epitaph)?).into()) } else { None };
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn abilities(self) -> &'static Vec<ActiveAbility> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Vec<ActiveAbility> = vec! $abilities;
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
    ТестоваяАктивка {
        name: "ТЕСТОВАЯ АКТИВКА",
        ru_gender: RuGender::Feminine,
        groups: [Group::Shiney],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnCharacter,
                conditions: vec![],

                description: "наносит 3 {dmg}".into(),

                callback: |game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(chr_id) = went_trigger else { unreachable!() };
                    game.hurt(chr_id, 3);
                }
            }
        ],
    }

/*     ПустаяКарта {
        name: "ПУСТАЯ КАРТА",
        ru_gender: RuGender::Feminine,
        groups: [Group::Shiney, Group::TBoI],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnField,
                conditions: vec![],

                description: "выбери активку в руке. эта карта повторит эффект выбранной"
                    .into(),

                callback: |game, self_id, _went_trigger| {
                    let owner_id = game.state().acts.find_owner(self_id).unwrap();
                    let imitated_act_id = game.choose_hand_act(owner_id);

                    todo!("mimic {:?}", imitated_act_id)
                }
            }
        ],
    } */
}
