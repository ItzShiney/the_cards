macro_rules! game_chaining_methods {
    (
        @ { $($arg_structs:tt)* }
        @ { $($game_callbacks_fields:tt)* }
        @ { $($game_state_impl:tt)* }

        $( #[chain($chain:ident)] )?
        try $method:ident ( &mut $self:ident $(, $arg:ident : $Arg:ty)* $(,)? ) $(-> $Ret:ty)? {
            can $can:block
            force $force:block
        }

        $($xs:tt)*
    ) => {::paste::paste! {
        game_chaining_methods! {
            @ {
                $($arg_structs)*

                pub struct [<$method:camel Args>] {
                    $(pub $arg: $Arg,)*
                }
            }
            @ {
                $($game_callbacks_fields)*

                pub [<can_ $method:camel:snake>]: Option<fn(&mut $crate::game::Game, [<$method:camel Args>]) -> Option<[<$method:camel Args>]>>,
                pub [<force_ $method:camel:snake>]: Option<fn(&mut $crate::game::Game, [<$method:camel Args>]) -> [<$method:camel Args>]>,
            }
            @ {
                $($game_state_impl)*

                pub fn [<can_ $method:camel:snake>] (&mut $self $(, $arg: $Arg)*) -> bool $can

                pub fn [<force_ $method:camel:snake>] (&mut $self $(, $arg: $Arg)*) $(-> $Ret)? $force

                #[allow(unused_parens)]
                pub fn [<try_ $method:camel:snake>] (&mut $self $(, $arg: $Arg)*) -> Option<($($Ret)?)> {
                    if $self.[<can_ $method:camel:snake>]($($arg),*) {
                        Some($self.[<force_ $method:camel:snake>]($($arg),*))
                    } else {
                        None
                    }
                }
            }
            $($xs)*
        }
    }};

    (
        @ { $($arg_structs:tt)* }
        @ { $($game_callbacks_fields:tt)* }
        @ { $($game_state_impl:tt)* }

        $( #[chain($chain:ident)] )?
        fn $method:ident ( &mut $self:ident $(, $arg:ident : $Arg:ty)* $(,)? ) $(-> $Ret:ty)? $body:block

        $($xs:tt)*
    ) => {::paste::paste! {
        game_chaining_methods! {
            @ {
                $($arg_structs)*

                pub struct [<$method:camel Args>] {
                    $(pub $arg: $Arg,)*
                }
            }
            @ {
                $($game_callbacks_fields)*

                pub $method: Option<fn($crate::game::Game, [<$method:camel Args>]) -> [<$method:camel Args>]>,
            }
            @ {
                $($game_state_impl)*

                #[allow(unused_parens)]
                pub fn $method (&mut $self $(, $arg: $Arg)*) $(-> $Ret)? $body
            }
            $($xs)*
        }
    }};

    (
        @ { $($arg_structs:tt)* }
        @ { $($game_callbacks_fields:tt)* }
        @ { $($game_state_impl:tt)* }
    ) => {
        $($arg_structs)*

        #[derive(Default)]
        pub struct GameCallbacks {
            $($game_callbacks_fields)*
        }

        impl $crate::game::Game<'_, '_> { $($game_state_impl)* }
    };

    (
        @ { $($arg_structs:tt)* }
        @ { $($game_callbacks_fields:tt)* }
        @ { $($game_state_impl:tt)* }

        $($xs:tt)*
    ) => {
        std::compile_error!(std::concat!("'", std::stringify!($($xs)*), "' could not match any branch"));
    };

    ( $($xs:tt)* ) => {
        game_chaining_methods!(
            @ {}
            @ {}
            @ {}
            $($xs)*
        );
    };
}
