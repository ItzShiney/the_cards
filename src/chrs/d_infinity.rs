pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["D INFINITY"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByЛёня,
        genders: [],
        tags: [TBoI],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 0/0/-0
    Stats::new(
        phy!(3?),
        dmg!(3?),
        int!(3?),
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["выставлен"]),
        Point(cs![
            Physique,
            " & ",
            Vitality,
            ", ",
            Damage,
            ", ",
            Intellect,
            " = ",
            Random(cs!["0"]..=cs!["9"]),
        ]),
    ]
}

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Place { chr_id: _chr_id } if _chr_id == chr_id => {
            let phy_vit = game.random(0, 9, chr_id);
            let dmg = game.random(0, 9, chr_id);
            let int = game.random(0, 9, chr_id);

            let set_phy = Event::stat_change(chr_id, StatType::Physique, StatChange::Set(phy_vit))
                .sign(chr_id)
                .try_(game);

            let set_vit = Event::stat_change(chr_id, StatType::Vitality, StatChange::Set(phy_vit))
                .sign(chr_id)
                .try_(game);

            let set_dmg = Event::stat_change(chr_id, StatType::Damage, StatChange::Set(dmg))
                .sign(chr_id)
                .try_(game);

            let set_int = Event::stat_change(chr_id, StatType::Intellect, StatChange::Set(int))
                .sign(chr_id)
                .try_(game);

            set_phy.or(set_vit).or(set_dmg).or(set_int)?;
        }

        _ => {}
    }

    Ok(signed_event)
}
