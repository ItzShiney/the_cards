pub use crate::chr_uses::*;

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

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Place { chr_id: _chr_id } if _chr_id == chr_id => {
            let Some(copied_chr_id) = game.choose_chr_in_hand_any(ChooseCardArgs {
                prompt: PromptArgs {
                    str: cs![
                        Character(Delirium),
                        ": чьи ",
                        Vitality,
                        " и ",
                        Damage,
                        " скопировать?"
                    ],
                    is_cancellable: true,
                    autochoose_single_option: false,
                },
                player_id: game.state.find_owner_of_chr(chr_id),
            }) else {
                return Err(Cancelled);
            };

            let stats = &game.state.chr(copied_chr_id).stats;
            let phy_vit = stats.phy.0;
            let dmg = stats.dmg.0;

            _ = Event::stat_change(chr_id, StatType::Physique, StatChange::Set(phy_vit))
                .sign(chr_id)
                .try_(game);

            _ = Event::stat_change(chr_id, StatType::Vitality, StatChange::Set(phy_vit))
                .sign(chr_id)
                .try_(game);

            _ = Event::stat_change(chr_id, StatType::Damage, StatChange::Set(dmg))
                .sign(chr_id)
                .try_(game);
        }

        _ => {}
    }

    Ok(signed_event)
}
