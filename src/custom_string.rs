use regex::Regex;
use std::fmt::Display;

macro_rules! custom_string_slice {
    (
        $($From:literal > $Name:ident > $Into:literal;)*
    ) => {
        pub enum CustomStringSlice {
            Raw(String),
            $($Name,)*
        }

        impl From<&str> for CustomStringSlice {
            fn from(value: &str) -> Self {
                match value {
                    $(concat!("{", $From, "}") => Self::$Name,)*
                    _ => Self::Raw(value.into()),
                }
            }
        }

        impl Display for CustomStringSlice {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Raw(c) => c,
                    $(Self::$Name => $Into,)*
                }
                .fmt(f)
            }
        }
    };
}

custom_string_slice![
    "=>" > Implies > "⟹";
    "*" > Bullet > "•";
    "x" > Mul > "⋅";
    "&" > And > "∧";

    "vit" > Vitality > "\x1b[31mVIT\x1b[39m";
    "phy" > Physique > "\x1b[35mPHY\x1b[39m";
    "def" > Defence > "\x1b[34mDEF\x1b[39m";
    "dmg" > Damage > "\x1b[33mDMG\x1b[39m";
    "int" > Intellect > "\x1b[36mINT\x1b[39m";
];

#[derive(Default)]
pub struct CustomString {
    slices: Vec<CustomStringSlice>,
}

impl From<&str> for CustomString {
    fn from(s: &str) -> Self {
        let regex = Regex::new("([^{]+|\\{[^}]*})").unwrap();

        let mut slices = Vec::default();
        for m in regex.find_iter(s) {
            let slice = m.as_str();
            slices.push(slice.into());
        }

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
