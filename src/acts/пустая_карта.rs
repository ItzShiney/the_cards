pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["ПУСТАЯ КАРТА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByМаксим,
        genders: [],
        tags: [TBoI, Иллюзия],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована"]),
        Point(cs!["выбери активку в руке. эта карта повторит эффект выбранной"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        can_use_on_field: Some(|_game, _args| {
            todo!();
        }),

        force_use_on_field: Some(|game, args| {
            let owner_id = game.state.find_owner_of_act(args.act_id);
            let imitated_act_id = game
                .choose_act_in_hand(ChooseCardArgsP {
                    prompt: PromptArgs {
                        str: cs![Active(ПустаяКарта), ": чей эффект повторить?"],
                        is_cancellable: false,
                        autochoose_single_option: false,
                    },
                    player_id: owner_id,
                    p: &|game_state, act_id| {
                        act_id != args.act_id && game_state.can_use_in_any_way(act_id)
                    },
                })
                .unwrap();

            todo!("повторить эффект {:?}", imitated_act_id)
        }),

        ..Default::default()
    }
}
