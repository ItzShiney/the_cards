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
        force_use_on_chr: Some(|game, args| {
            let target_owner_id = game.state.find_owner_of_chr(args.target_id);

            let replacing_chr_id = game
                .choose_chr_in_hand(ChooseCardArgsP {
                    prompt: PromptArgs {
                        str: cs![Active(Берн), ": на кого поменять?"],
                        is_cancellable: false,
                        autochoose_single_option: true,
                    },
                    player_id: target_owner_id,
                    p: &|game, chr_id| chr_id != args.target_id && game.can_place(chr_id),
                })
                .unwrap();

            game.replace(args.target_id, replacing_chr_id);
            args
        }),

        ..Default::default()
    }
}
