use crate::custom_string::CustomString;
use crate::game_state::ability::active_ability::{ActiveAbility, ActiveTrigger, WentActiveTrigger};
use crate::game_state::group::Group;
use crate::gendered::RuGender;

use std::collections::BTreeSet;

macro_rules! acts {
    (
        $(
            $CardName:ident {
                const NAME = $name:literal;
                const RU_GENDER = $ru_gender:expr;
                const GROUPS = $groups:tt;

                const ABILITIES = $abilities:tt;
            }
        )*
    ) => {paste::paste!{
        #[derive(Clone, Copy)]
        pub enum ActiveType {
            $($CardName,)*
        }

        impl ActiveType {
            pub fn all() -> Vec<ActiveType> {
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
    TestActive {
        const NAME = "ТЕСТОВАЯ АКТИВКА";
        const RU_GENDER = RuGender::Feminine;
        const GROUPS = [];

        const ABILITIES = [
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
        ];
    }

    EmptyCard {
        const NAME = "ПУСТАЯ КАРТА";
        const RU_GENDER = RuGender::Feminine;
        const GROUPS = [Group::TBoI];

        const ABILITIES = [
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
        ];
    }
}
