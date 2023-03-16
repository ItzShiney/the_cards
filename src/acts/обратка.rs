pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["О,БРАТКА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder { 
        tier: A, 
        author: ByЛёша, 
        genders: [], 
        tags: [],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на противника, единственного на поле"]),
        Point(cs!["персонаж противника становится твоим и выставляется от тебя"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        use_on_chr: Some(|game, args| {
            let owner_id = game.state().try_find_owner_act(args.act_id);
            let target_owner_id = game.state().try_find_owner_chr(args.target_id);

            if owner_id == target_owner_id {
                return Break(Err(Terminated));
            }

            todo!()
        }),

        ..Default::default()
    }
}
