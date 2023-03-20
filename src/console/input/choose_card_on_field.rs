macro_rules! choose_card_on_field {
    ( $Namespace:ident, $namespace:ident, |$state:ident| $options:expr ) => {::paste::paste!{
        fn [<choose_ $namespace _on_field>]<'p>(
            &'p mut self,
            $state: &'p mut $crate::game_state::GameState,
            args: $crate::game_input::ChooseCardArgsP<'p, $crate::game_state::[<$namespace _id>]::[<$Namespace ID>]>,
        ) -> Option<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> {
            let cards: Vec<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> = $options;

            let mut game = $crate::game::Game { state: $state, input: self };
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
            Some(cards[idx])
        }
    }};
}
