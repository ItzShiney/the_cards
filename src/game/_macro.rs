#[macro_export]
macro_rules! callbacks {
    (
        $(
            $(#[ping($self_namespace:ident)])?
            $(#[pre($pre_value:expr)])?
            pub fn $name:ident(
                &mut $self:ident
                $(
                    , $arg_name:ident : $ArgType:ty
                )* $(,)?
            ) $(-> $Return:ty)? $callback_action:block
        )*
    ) => {paste::paste! {
        $(
            #[derive(Clone)]
            pub struct [<$name:camel Args>] {
                $(pub $arg_name: $ArgType,)*
            }

            pub type [<$name:camel Callback>] = fn(&mut $crate::game::Game, [<$name:camel Args>]) -> $crate::game::chain::Chain<[<$name:camel Args>], $($Return)?>;
            pub type [<Post $name:camel Callback>] = fn(&mut $crate::game::Game, &[<$name:camel Args>]);
        )*

        #[derive(Default)]
        pub struct GameCallbacks {
            $(
                pub $name: Option<[<$name:camel Callback>]>,
                pub [<post_ $name>]: Option<[<Post $name:camel Callback>]>,
            )*
        }

        impl $crate::game::Game {
            $(
                pub fn [<$name:camel:snake _args>] (&mut $self, #[allow(unused_mut)] mut args: [<$name:camel Args>] ) $(-> $Return)? {
                    /* $(
                        const _: () = assert!($pre_value);

                        while let Some(callback) = $self.callbacks.$name {
                            match (callback)($self, args) {
                                $crate::game::chain::Chain::Continue(new_args) => {
                                    args = new_args;
                                }

                                $crate::game::chain::Chain::Break(result) => return result,
                            }
                        }
                    )? */

                    #[allow(unused)] let id = ($(args.$arg_name,)* 0,).0;
                    $(
                        if let Some(callback) = $self.state.$self_namespace.get(id).type_.abilities().$name {
                            match (callback)($self, args) {
                                $crate::game::chain::Chain::Continue(new_args) => {
                                    args = new_args;
                                }

                                $crate::game::chain::Chain::Break(result) => return result,
                            }
                        }
                    )?
                    #[allow(unused)] let id = ();

                    #[allow(clippy::redundant_closure_call)]
                    let res = (|| {
                        #[allow(unused)]
                        let [<$name:camel Args>] { $($arg_name,)* } = args.clone();

                        $callback_action
                    })();

                    $self.[<post_ $name:camel:snake _args>](&args);
                    res
                }

                pub fn [<post_ $name:camel:snake _args>] (&mut $self, #[allow(unused)] args: &[<$name:camel Args>] ) {
                    /* while let Some(callback) = $self.callbacks.[<post_ $name>] {
                        (callback)($self, args);
                    } */

                    #[allow(unused)] let id = ($(args.$arg_name,)* 0,).0;
                    $(
                        if let Some(callback) = $self.state.$self_namespace.get(id).type_.abilities().[<post_ $name>] {
                            (callback)($self, args);
                        }
                    )?
                    #[allow(unused)] let id = ();
                }

                pub fn $name (&mut $self, $($arg_name: $ArgType,)* ) $(-> $Return)? {
                    $self.[<$name:camel:snake _args>]([<$name:camel Args>] { $($arg_name,)* })
                }
            )*
        }
    }};
}
