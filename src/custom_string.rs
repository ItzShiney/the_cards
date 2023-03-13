use crate::acts::ActiveType;
use crate::chrs::CharacterType;
use crate::group::Group;
use itertools::Itertools;
use std::fmt::Display;
use std::ops::RangeInclusive;

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
        #[derive(Clone)]
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
            |f, (str)| write!(f, "{}", str);

        Character(CharacterType) =>
            |f, (type_)| write!(f, "\x1b[1m[{}]\x1b[0m", type_.name());

        Active(ActiveType) =>
            |f, (type_)| write!(f, "\x1b[1m[{}]\x1b[0m", type_.name());

        Group(Group) =>
            |f, (group)| write!(f, "{}", group);

        SumAll { body: CustomString } =>
            |f, { body }| write!(f, "∑ {}", body);

        Sum { times: CustomString, body: CustomString } =>
            |f, { times, body }| write!(f, "∑[{} раз] {}", times, body);

        Random(RangeInclusive<CustomString>) =>
            |f, (range)| write!(f, "🎲[{}..{}]", range.start(), range.end());

        Mean(Vec<CustomString>) =>
            |f, (args)| write!(f, "⟨{}⟩", args.iter().join(", "));

        Epitaph(CustomString) =>
            |f, (line)| writeln!(f, "\x1b[3m{}\x1b[0m", line);

        Condition(CustomString) =>
            |f, (condition)| writeln!(f, "{} {}", condition, Self::Implies);

        Point(CustomString) =>
            |f, (line)| writeln!(f, "{} {}", Self::Bullet, line);

        NamedPoint(CustomString, CustomString) =>
            |f, (name, line)| writeln!(f, "{} {}: {}", Self::Bullet, Self::Name(name.clone()), line);

        Name(CustomString) =>
            |f, (line)| write!(f, "\x1b[1m{}\x1b[22m", line);
    }

    {
        Activatable => |f| writeln!(f, "активируемая способность:");

        Implies => |f| write!(f, "⟹ ");
        Bullet => |f| write!(f, "•");
        Mul => |f| write!(f, "⋅");

        Vitality => |f| write!(f, "\x1b[31mVIT\x1b[39m");
        Physique => |f| write!(f, "\x1b[35mPHY\x1b[39m");
        Defence => |f| write!(f, "\x1b[34mDEF\x1b[39m");
        Damage => |f| write!(f, "\x1b[33mDMG\x1b[39m");
        Intellect => |f| write!(f, "\x1b[36mINT\x1b[39m");

        LE => |f| write!(f, "≤");
        GE => |f| write!(f, "≥");

        __ => |f| writeln!(f);
    }
];

impl From<&str> for CustomStringSlice {
    fn from(raw: &str) -> Self {
        Self::Raw(raw.into())
    }
}

impl From<String> for CustomStringSlice {
    fn from(raw: String) -> Self {
        Self::Raw(raw)
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

impl From<Group> for CustomStringSlice {
    fn from(group: Group) -> Self {
        Self::Group(group)
    }
}

#[derive(Clone, Default)]
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
    ($($args:expr),* $(,)?) => {{
        #[allow(unused)] use $crate::custom_string::CustomStringSlice::*;
        #[allow(unused)] use $crate::chrs::CharacterType::*;
        #[allow(unused)] use $crate::acts::ActiveType::*;
        #[allow(unused)] use $crate::group::Group::*;
        $crate::custom_string::CustomString::from(vec![$($args.into()),*])
    }};
}
