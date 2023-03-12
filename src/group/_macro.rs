#[macro_export]
macro_rules! group {
    (
        $(
            $Name:ident > $Into:literal $(: [
                $($Super:ident),*
            ])?;
        )*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Group {
            $($Name,)*
        }

        impl Group {
            pub fn supers(self) -> Vec<Group> {
                match self {
                    $(Self::$Name => vec![$($(Self::$Super),*)?],)*
                }
            }
        }

        impl Display for Group {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "\x1b[1m[{}]\x1b[22m",
                    match self {
                        $(Self::$Name => $Into,)*
                    }
                )
            }
        }
    };
}
