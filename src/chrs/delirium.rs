pub use crate::card_uses::*;

pub fn name() -> CustomString {
    cs!["DELIRIUM"]
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

#[rustfmt::skip]
pub fn stats() -> Stats {
    // ?/?/0
    Stats::new(
        phy!(5?),
        dmg!(5?),
        int!(0), // представляет собой безумие
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["выставлен"]),
        Point(cs![
            "выбери персонажа в руке. ",
            Vitality,
            " = его ",
            Vitality,
            ", ",
            Damage,
            " = его ",
            Damage
        ])
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        post_place: Some(|game, args| {
            let self_id = args.chr_id;
            let owner_id = game.state().find_owner_chr(self_id);
            let Some(copied_chr_id) = game.choose_chr_in_hand_any(ChooseCardArgs {
                prompt: PromptArgs {
                    str: cs![Character(Delirium), ": чьи ", Vitality, " и ", Damage, " скопировать?"],
                    is_cancellable: true,
                    autochoose_single_option: false,
                },
                player_id: owner_id,
            }) else { return };

            let stats = &game.state().chr(copied_chr_id).stats;
            let phy = stats.phy.0.into_value();
            let dmg = stats.dmg.0.into_value();

            game.force_set_phy_vit(self_id, phy);
            game.force_set_stat(self_id, StatType::Damage, dmg);
        }),

        ..Default::default()
    }
}
