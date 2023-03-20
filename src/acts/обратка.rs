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
        can_use_on_chr: Some(|game, args| {
            let is_used_on_enemy =
                Some(args.target_id) == game.state.other_subturner_on_field().chr_id;
            let is_enemy_single = game.state.current_subturner_on_field().chr_id.is_none();

            (is_used_on_enemy && is_enemy_single).then_some(args)
        }),

        force_use_on_chr: Some(|game, args| {
            todo!();
        }),

        ..Default::default()
    }
}
