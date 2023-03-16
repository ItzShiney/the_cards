pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["БОРАТ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByКостя,
        genders: [Мужчина],
        tags: [Мемы],
    }.into()
}

// 2/2/-4
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(4),
        dmg!(3),
        int!(1),
    )
}

pub fn description() -> CustomString {
    cs![
    Condition(cs!["выставлен"]),
    NamedPoint(cs!["\"Я РЕПОРТЁР ИЗ КАЗАХСТАНА\""], cs!["возьми активку из стопки добора. если возможно, используй на этого персонажа, иначе положи обратно"]),
]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        post_place: Some(|game, args| {
            let self_id = args.chr_id;
            let owner_id = game.state().find_owner_chr(self_id);

            let Some(gained_act_id) = game.state_mut().acts.pick(owner_id) else { return };
            if game.use_on_chr(gained_act_id, self_id).is_err() {
                game.state_mut().acts.add_to_drawpile(gained_act_id);
            }
        }),

        ..Default::default()
    }
}
