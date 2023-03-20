macro_rules! choose_card_on_field {
    ( $Namespace:ident, $namespace:ident, |$game_state:ident| $options:expr ) => {::paste::paste!{
        pub struct [<Choose $Namespace OnField>];

        impl $crate::game_input::[<Choose $namespace:camel OnField>] for [<Choose $Namespace OnField>] {
            fn [<choose_ $namespace _on_field>] <'game_state>(
                &mut self,
                $game_state: &'game_state $crate::game_state::GameState,
                args: $crate::game_input::ChooseCardArgsP<'_, $crate::game_state::[<$namespace _id>]::[<$Namespace ID>]>,
            ) -> Option<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> {
                use itertools::Itertools;

                let cards: Vec<$crate::game_state::[<$namespace _id>]::[<$Namespace ID>]> = $options;
                let is_enabled =
                    cards.iter().copied().map(|id| (args.p)($game_state, id)).collect_vec();

                let displays = cards
                    .clone()
                    .into_iter()
                    .map(|id| $crate::cs![$Namespace ($game_state.$namespace(id).type_)]);

                let idx = $crate::console::prompt(
                    args.prompt,
                    is_enabled.iter().copied().zip(displays),
                )?;
                Some(cards[idx])
            }
        }
    }};
}

choose_card_on_field!(Character, chr, |game_state| {
    let mut res = vec![];

    if let Some(chr_id) = game_state.attacker.chr_id {
        res.push(chr_id);
    }

    if let Some(chr_id) = game_state.defender.chr_id {
        res.push(chr_id);
    }

    res
});

choose_card_on_field!(Active, act, |game_state| {
    game_state
        .attacker
        .used_act_ids
        .iter()
        .copied()
        .chain(game_state.defender.used_act_ids.iter().copied())
        .collect()
});
