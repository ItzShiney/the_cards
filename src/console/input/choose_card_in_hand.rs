macro_rules! choose_card_in_hand {
    ( $Namespace:ident, $namespace:ident ) => {::paste::paste!{
        fn [<choose_ $namespace _in_hand>]<'p>(
            &'p mut self,
            state: &'p mut $crate::game_state::GameState,
            args: $crate::game_input::ChooseCardArgsP<'p, $crate::game_state::[<$namespace _id>]::[<$Namespace ID>]>,
        ) -> Option<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> {
            let mut game = $crate::game::Game { state, input: self };

            let cards = game.state.[<$namespace s>].hand(args.player_id).clone();

            let mut is_enabled = vec![];
            let mut displays = vec![];
            for id in cards.iter().copied() {
                is_enabled.push((args.p)(&mut game, id));
                displays.push($crate::cs![$Namespace (game.state.$namespace(id).type_)]);
            }

            let idx = $crate::console::prompt(
                args.prompt,
                is_enabled.iter().copied().zip(displays),
            )?;
            Some(cards.iter().copied().nth(idx).unwrap())
        }
    }};
}
