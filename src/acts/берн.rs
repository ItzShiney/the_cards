use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БЕРН"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByМаксим,
        genders: [],
        tags: [Umineko],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на противника, единственного на поле"]),
        Point(cs!["противник обязан поменять персонажа"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_chr: Some(|game, args| {
            let target_owner_id = game.state().find_owner_chr(args.target_id);

            let Some(replacing_chr_id) = game.choose_chr_in_hand(ChooseCardArgsP {
            prompt: PromptArgs {
                str: cs![Active(Берн), ": на кого поменять?"],
                is_cancellable: false,
                autochoose_single_option: true,
            },
            player_id: target_owner_id,
            p: &|game_state, chr_id| chr_id != args.target_id && game_state.is_placeable(chr_id)
        }) else { return Break(Err(Terminated)) };

            game.replace(args.target_id, replacing_chr_id);
            Continue(args)
        }),

        ..Default::default()
    }
}
