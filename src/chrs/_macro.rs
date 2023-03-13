#[macro_export]
macro_rules! chrs {
    (
        $(
            $CardName:ident {
                name: $name:expr,
                groups: $groups:tt,

                stats: $stats:expr,

                $(description: $description:expr,)?

                $(abilities: $abilities:expr)? $(,)?
            }
        )*
    ) => {::paste::paste!{
        #[derive(Clone, Copy)]
        pub enum CharacterType {
            $($CardName,)*
        }

        impl CharacterType {
            pub fn all() -> Vec<Self> {
                vec![
                    $(Self::$CardName,)*
                ]
            }

            pub fn name(self) -> &'static $crate::custom_string::CustomString {
                ::lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::custom_string::CustomString = $name.into();
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn groups(self) -> &'static ::std::collections::BTreeSet<$crate::group::Group> {
                #[allow(unused)]
                use $crate::group::Group::*;

                ::lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: ::std::collections::BTreeSet<$crate::group::Group> = {
                            const _: () = {
                                if !matches!($groups[0], $crate::group::Group::S | $crate::group::Group::A | $crate::group::Group::B | $crate::group::Group::C | $crate::group::Group::D) {
                                    panic!(concat!(stringify!($CardName), ".groups[0] is not a tier"));
                                }
                            };

                            const _: () = {
                                if !matches!($groups[1], $crate::group::Group::ByЛёня | $crate::group::Group::ByМаксим | $crate::group::Group::ByКостя | $crate::group::Group::ByЛёша) {
                                    panic!(concat!(stringify!($CardName), ".groups[1] is not a creator"));
                                }
                            };

                            ::std::collections::BTreeSet::from($groups)
                        };
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }

            pub fn description(self) -> &'static Option<$crate::custom_string::CustomString> {
                ::lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Option<$crate::custom_string::CustomString> =  {
                            let x = (
                                $($description,)?
                                $crate::cs![],
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

            pub fn stats(self) -> $crate::stats::Stats {
                match self {
                    $(Self::$CardName => $stats,)*
                }
            }

            pub fn abilities(self) -> &'static $crate::game::GameCallbacks {
                use $crate::group::Group::*;

                ::lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::game::GameCallbacks =
                            (
                                $($abilities,)?
                                $crate::game::GameCallbacks::default(),
                            ).0;
                    )*
                }

                match self {
                    $(Self::$CardName => &[<$CardName:snake:upper>],)*
                }
            }
        }
    }};
}
