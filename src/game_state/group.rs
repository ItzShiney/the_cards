use std::collections::BTreeSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::default_formatted::DefaultFormatted;

macro_rules! groups {
    (
        $(
            $Name:ident > $Into:literal : [
                $($Super:ident),*
            ];
        )*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Group {
            $($Name,)*
        }

        impl Group {
            pub fn supers(self) -> BTreeSet<Group> {
                match self {
                    $(Self::$Name => [$(Self::$Super),*].into(),)*
                }
            }
        }

        impl Display for Group {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$Name => $Into,)*
                }
                .fmt(f)
            }
        }
    };
}

groups![
    TBoI > "tboi": [Games];
    Games > "games": [];
    Reality > "reality": [];
];

impl Display for DefaultFormatted<&BTreeSet<Group>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            self.0.iter().map(|&x| format!("[{x}]")).join(" ").fmt(f)?;
            writeln!(f)
        } else {
            Ok(())
        }
    }
}
