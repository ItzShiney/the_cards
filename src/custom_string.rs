use std::{fmt::Display, ops::RangeInclusive};

use itertools::Itertools;

use crate::{acts::ActiveType, chrs::CharacterType, game_state::group::Group};

macro_rules! custom_string_slice {
    (
        {
            $(
                $ArgsCase:ident $args:tt
                    => |$args_formatter:ident, $new_args:tt| $args_custom_fmt:expr;
            )*
        }

        {
            $(
                $Case:ident
                    => |$formatter:ident| $custom_fmt:expr;
            )*
        }
    ) => {
        pub enum CustomStringSlice {
            $($ArgsCase $args,)*

            $($Case,)*
        }

        impl Display for CustomStringSlice {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$ArgsCase $new_args => {
                        let $args_formatter = f;
                        $args_custom_fmt
                    },)*

                    $(Self::$Case => {
                        let $formatter = f;
                        $custom_fmt
                    },)*
                }
            }
        }
    };
}

custom_string_slice![
    {
        Raw(String) =>
            |f, (__0)| write!(f, "{}", __0);

        Character(CharacterType) =>
            |f, (type_)| write!(f, "\x1b[1m[{}]\x1b[0m", type_.name());

        Active(ActiveType) =>
            |f, (type_)| write!(f, "\x1b[1m[{}]\x1b[0m", type_.name());

        Group(Group) =>
            |f, (group)| write!(f, "{}", group);

        Sum { body: CustomString } =>
            |f, { body }| write!(f, "∑ {}", body);

        SumTimes { times: CustomString, body: CustomString } =>
            |f, { times, body }| write!(f, "∑[{} раз] {}", times, body);

        Random(RangeInclusive<CustomString>) =>
            |f, (range)| write!(f, "🎲[{}..{}]", range.start(), range.end());

        Mean(Vec<CustomString>) =>
            |f, (args)| write!(f, "⟨{}⟩", args.into_iter().join(", "));
    }

    {
        Implies => |f| write!(f, "⟹");
        Bullet => |f| write!(f, "•");
        Mul => |f| write!(f, "⋅");
        And => |f| write!(f, "∧");

        Vitality => |f| write!(f, "\x1b[31mVIT\x1b[39m");
        Physique => |f| write!(f, "\x1b[35mPHY\x1b[39m");
        Defence => |f| write!(f, "\x1b[34mDEF\x1b[39m");
        Damage => |f| write!(f, "\x1b[33mDMG\x1b[39m");
        Intellect => |f| write!(f, "\x1b[36mINT\x1b[39m");
    }
];

impl From<&str> for CustomStringSlice {
    fn from(raw: &str) -> Self {
        Self::Raw(raw.into())
    }
}

impl From<CharacterType> for CustomStringSlice {
    fn from(type_: CharacterType) -> Self {
        Self::Character(type_)
    }
}

impl From<ActiveType> for CustomStringSlice {
    fn from(type_: ActiveType) -> Self {
        Self::Active(type_)
    }
}

#[derive(Default)]
pub struct CustomString {
    pub slices: Vec<CustomStringSlice>,
}

impl From<Vec<CustomStringSlice>> for CustomString {
    fn from(slices: Vec<CustomStringSlice>) -> Self {
        Self { slices }
    }
}

impl Display for CustomString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.slices.iter() {
            c.fmt(f)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! cs {
    ($($args:expr),*) => {{
        #[allow(unused)] use $crate::custom_string::CustomStringSlice::*;
        #[allow(unused)] use $crate::chrs::CharacterType::*;
        #[allow(unused)] use $crate::acts::ActiveType::*;
        #[allow(unused)] use $crate::game_state::group::Group::*;
        $crate::custom_string::CustomString::from(vec![$($args.into()),*])
    }};
}
