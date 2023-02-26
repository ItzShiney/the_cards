use std::fmt::Display;

use crate::{acts::ActiveType, chrs::CharacterType};

macro_rules! custom_string_slice {
    (
        {$(
            $CustomName:ident $args:tt
                => |$formatter:ident, $new_args:tt| $custom_fmt:expr;
        )*}

        {$(
            $Name:ident => $Into:literal;
        )*}
    ) => {
        pub enum CustomStringSlice {
            $($CustomName $args,)*
            $($Name,)*
        }

        impl Display for CustomStringSlice {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$CustomName $new_args => {
                        let $formatter = f;
                        $custom_fmt
                    },)*

                    $(Self::$Name => $Into.fmt(f),)*
                }
            }
        }
    };
}

custom_string_slice![
    {
        Raw(String)
            => |f, (__0)| write!(f, "{}", __0);

        Character(CharacterType)
            => |f, (type_)| write!(f, "\x1b[1m{}\x1b[0m", type_.name());

        Active(ActiveType)
            => |f, (type_)| write!(f, "\x1b[1m{}\x1b[0m", type_.name());

        Sum { times: CustomString, body: CustomString }
            => |f, { times, body }| write!(f, "∑[{} раз] {}", times, body);

        Random { min: CustomString, max: CustomString }
            => |f, { min, max }| write!(f, "🎲[{}..{}]", min, max);
    }

    {
        Implies => "⟹";
        Bullet => "•";
        Mul => "⋅";
        And => "∧";

        Vitality => "\x1b[31mVIT\x1b[39m";
        Physique => "\x1b[35mPHY\x1b[39m";
        Defence => "\x1b[34mDEF\x1b[39m";
        Damage => "\x1b[33mDMG\x1b[39m";
        Intellect => "\x1b[36mINT\x1b[39m";
    }
];

#[derive(Default)]
pub struct CustomString {
    pub slices: Vec<CustomStringSlice>,
}

#[macro_export]
macro_rules! __cs_helper {
    () => {{
        let x: [$crate::custom_string::CustomStringSlice; 0] = [];
        x.into_iter()
    }};

    ($str:literal $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::Raw(String::from($str))]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };

    (Character($CharacterType:ident) $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::Character($crate::chrs::CharacterType::$CharacterType)]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };

    (Active($ActiveType:ident) $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::Active($crate::acts::ActiveType::$ActiveType)]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };

    (Sum($times:expr, $body:expr) $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::Sum { times: $times, body: $body }]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };

    (Random($min:expr, $max:expr) $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::Random { min: $min, max: $max }]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };

    ($EnumCase:ident $($xs:tt)*) => {
        [$crate::custom_string::CustomStringSlice::$EnumCase]
            .into_iter().chain($crate::__cs_helper![$($xs)*])
    };
}

#[macro_export]
macro_rules! cs {
    ($($xs:tt)*) => {
        $crate::custom_string::CustomString::from($crate::__cs_helper![$($xs)*].collect::<Vec<$crate::custom_string::CustomStringSlice>>())
    };
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
