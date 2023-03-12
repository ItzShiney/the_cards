macro_rules! choose_card_in_hand {
    ( $Namespace:ident, $namespace:ident ) => {::paste::paste!{
        pub struct [<Choose $Namespace InHand>];

        impl $crate::game::input::[<Choose $namespace:camel InHand>] for [<Choose $Namespace InHand>] {
            fn [<choose_ $namespace _in_hand>] <'game_state>(
                &mut self,
                game_state: &'game_state $crate::game::state::GameState,
                args: $crate::game::input::ChooseCardArgsP<'_, 'game_state, '_, $crate::game::state::[<$namespace _id>]::[<$Namespace ID>]>,
            ) -> Option<$crate::game::state::[<$namespace _id>]::[<$Namespace ID>]> {
                use itertools::Itertools;

                let cards = game_state.[<$namespace s>].hand(args.player_id).clone();
                let is_enabled =
                    cards.iter().copied().map(|id| (args.p)(game_state, id)).collect_vec();

                let displays = cards
                    .clone()
                    .into_iter()
                    .map(|id| $crate::cs![$Namespace (game_state.$namespace(id).type_)]);

                let idx = $crate::console::prompt(
                    args.prompt,
                    args.is_cancellable,
                    is_enabled.iter().copied().zip(displays),
                )?;
                Some(cards[idx])
            }
        }
    }};
}

choose_card_in_hand!(Character, chr);
choose_card_in_hand!(Active, act);
