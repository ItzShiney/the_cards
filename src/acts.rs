#[allow(unused)]
use crate::{
    chrs::CharacterType,
    cs,
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
                name: $name:expr,
                ru_gender: $ru_gender:expr,
                groups: $groups:tt,

                $(epitaph: $epitaph:expr,)?

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
                        static ref [<$CardName:snake:upper>]: CustomString = $name;
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
    ПустаяКарта {
        name: cs!["ПУСТАЯ КАРТА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnField,
                conditions: vec![],

                description: cs!["выбери активку в руке. эта карта повторит эффект выбранной"]
                    .into(),

                callback: |game, self_id, _went_trigger| {
                    let owner_id = game.state().acts.find_owner(self_id).unwrap();
                    let imitated_act_id = game.choose_hand_act(owner_id);

                    todo!("mimic {:?}", imitated_act_id)
                }
            }
        ],
    }

    /*
    Баян {
        name: cs!["БАЯН"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByMaxvog, Group::Dismoral],

        abilities: [
            ActiveAbility {
                name: Some(cs!["\"ЭТОТ АНЕКДОТ ЕЩЁ МОЙ ДЕД МОЕМУ ОТЦУ РАССКАЗЫВАЛ\""].into()),

                trigger: ActiveTrigger::UsedOnEnemyCharacter,
                conditions: vec![],

                description: cs!["{dmg} -= 3"].into(),

                callback: |game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(chr_id) = went_trigger else { unreachable!() };
                    game.sub_dmg(chr_id, 3);
                }
            }
        ],
    }

    ЖёлтаяИскра {
        name: cs!["ЖЁЛТАЯ ИСКРА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::Undertale],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnCharacter,
                conditions: vec![],

                description: cs!["{vit} = {phy}"].into(),

                callback: |game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(chr_id) = went_trigger else { unreachable!() };

                    let phy = game.state().chr(chr_id).stats.phy.0.into_value().unwrap();
                    game.set_vit(chr_id, phy);
                }
            }
        ],
    }

    ТетрадьСмерти {
        name: cs!["ТЕТРАДЬ СМЕРТИ"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByConstantine, Group::DeathNote],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnCharacter,
                conditions: vec![],

                description: cs!["мгновенно убивает его"].into(),

                callback: |_game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(_chr_id) = went_trigger else { unreachable!() };

                    todo!()
                }
            }
        ],
    }

    Коммунизм {
        name: cs!["КОММУНИЗМ"],
        ru_gender: RuGender::Masculine,
        groups: [Group::ByConstantine, Group::SocialOrder],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedAsTurn,
                conditions: vec![],

                description: cs!["каждый игрок передаёт свою колоду следующему по направлению ходов. эта карта уничтожается"].into(),

                callback: |_game, _self_id, _went_trigger| {
                    todo!()
                }
            }
        ],
    }

    ОБратка {
        name: cs!["О,БРАТКА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByZoinX],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnEnemyCharacter,
                conditions: vec![],

                description: cs!["персонаж выставляется как твой"].into(),

                callback: |_game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(_chr_id) = went_trigger else { unreachable!() };

                    todo!()
                }
            }
        ],
    }

    ЛезвиеНожа {
        name: cs!["ЛЕЗВИЕ НОЖА"],
        ru_gender: RuGender::Neuter,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnCharacter,
                conditions: vec![],

                // FIXME
                description: cs![
                    Damage, " += 1\n",
                    Bullet, " если ранее была использована ", РучкаНожа, ", получи ", Нож
                ],

                callback: |_game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(_chr_id) = went_trigger else { unreachable!() };

                    todo!()
                }
            },
        ],
    }

    РучкаНожа {
        name: cs!["РУЧКА НОЖА"],
        ru_gender: RuGender::Feminine,
        groups: [Group::ByShiney, Group::TBoI],

        abilities: [
            ActiveAbility {
                name: None,

                trigger: ActiveTrigger::UsedOnCharacter,
                conditions: vec![],

                // FIXME
                description: cs![
                    Physique, " += 1\n",
                    Bullet, " если ранее было использовано ", ЛезвиеНожа, ", получи ", Нож
                ],

                callback: |_game, _self_id, went_trigger| {
                    let WentActiveTrigger::UsedOnCharacter(_chr_id) = went_trigger else { unreachable!() };

                    todo!()
                }
            },
        ],
    }
    // */
}
