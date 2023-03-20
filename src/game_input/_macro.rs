#[macro_export]
macro_rules! game_input {
    (
        $(
            fn $method:ident
            $(<
                $($lt:lifetime),+
                $(,)?
            >)?
            (
                &mut self,
                $( $arg:ident : $Arg:ty ),*
                $(,)?
            )
                $(-> $Return:ty)?;
        )*
    ) => {::paste::paste!{
        $(
            pub trait [<$method:camel>] {
                fn $method $(<$($lt),+>)? ( &mut self, $( $arg: $Arg ),* ) $(-> $Return)?;
            }

            #[allow(unused)]
            impl<T: Iterator<Item = ($($Return)?)>> [<$method:camel>] for T {
                fn $method $(<$($lt),+>)? (&mut self, $( $arg: $Arg ),*) $(-> $Return)? {
                    self.next().unwrap()
                }
            }
        )*

        pub trait GameInput: $([<$method:camel>] +)* {}

        pub struct GameInputTuple<
            $(
                [<Tuple $method:camel>]: [<$method:camel>],
            )*
        > {
            $(
                pub $method: [<Tuple $method:camel>],
            )*
        }

        macro_rules! __impl_game_input {
            (
                $Trait:ident,
                $body:tt
            ) => {
                impl<
                    $(
                        [<Tuple $method:camel>]: [<$method:camel>],
                    )*
                > $Trait for GameInputTuple<
                    $(
                        [<Tuple $method:camel>],
                    )*
                > $body
            }
        }

        __impl_game_input!(GameInput, {});

        $(
            __impl_game_input!([<$method:camel>], {
                fn $method $(<$($lt),+>)? (&mut self, $($arg: $Arg),*) $(-> $Return)? {
                    self.$method.$method($($arg),*)
                }
            });
        )*
    }};
}
