#[macro_export]
macro_rules! acts {
    (
        $(
            $CardName:ident {
                name: $name:expr,
                groups: $groups:tt,

                $(description: $description:expr,)?

                abilities: $abilities:expr,
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

            pub fn name(self) -> &'static $crate::custom_string::CustomString {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::custom_string::CustomString = $name;
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
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

            pub fn description(self) -> &'static Option<CustomString> {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: Option<CustomString> = {
                            let x = (
                                $($description,)?
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

            pub fn abilities(self) -> &'static $crate::host::GameCallbacks {
                lazy_static::lazy_static! {
                    $(
                        static ref [<$CardName:snake:upper>]: $crate::host::GameCallbacks = $abilities;
                    )*
                }

                match self {
                    $(Self::$CardName => &*[<$CardName:snake:upper>],)*
                }
            }
        }
    }};
}
