macro_rules! choose_card_in_hand {
    ( $Namespace:ident, $namespace:ident ) => {::paste::paste!{
        pub struct [<Choose $Namespace InHand>];

        impl $crate::game_input::[<Choose $namespace:camel InHand>] for [<Choose $Namespace InHand>] {
            fn [<choose_ $namespace _in_hand>] <'game_state>(
                &mut self,
                game_state: &'game_state mut $crate::game_state::GameState,
                args: $crate::game_input::ChooseCardArgsP<'_, $crate::game_state::[<$namespace _id>]::[<$Namespace ID>]>,
            ) -> Option<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> {
                use itertools::Itertools;

                let cards = game_state.[<$namespace s>].hand(args.player_id).clone();
                let is_enabled =
                    cards.iter().copied().map(|id| (args.p)($crate::game::Game { state: game_state, input: self }, id)).collect_vec();

                let displays = cards
                    .clone()
                    .into_iter()
                    .map(|id| $crate::cs![$Namespace (game_state.$namespace(id).type_)]);

                let idx = $crate::console::prompt(
                    args.prompt,
                    is_enabled.iter().copied().zip(displays),
                )?;
                Some(cards[idx])
            }
        }
    }};
}

choose_card_in_hand!(Character, chr);
choose_card_in_hand!(Active, act);
