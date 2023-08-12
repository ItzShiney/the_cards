pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["БАНКА С ВАРЕНЬЕМ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [Реальность],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 1/3/-0
    Stats::new(
        phy!(1), // легко разбивается
        dmg!(2), // бьёт осколками
        int!(0),
    )
}

pub fn description() -> CustomString {
    cs![Point(cs![
        "не атакует, если ",
        Intellect,
        " атакуемого ",
        GE,
        " 3"
    ])]
}

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Attack {
            attacker_id,
            target_id,
            ..
        } if attacker_id == chr_id => {
            let enemy_int = game.stat(target_id, StatType::Intellect, chr_id);
            if enemy_int >= 3 {
                return Err(Cancelled("[БАНКА С ВАРЕНЬЕМ]: int >= 3"));
            }
        }

        _ => {}
    }

    Ok(signed_event)
}
