pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["БЕАТРИЧЕ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByМаксим,
        genders: [Женщина],
        tags: [Umineko, Иллюзия],
    }.into()
}

// 1/4/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(5),
        dmg!(8),
        int!(7),
    )
}

pub fn description() -> CustomString {
    cs![Point(cs!["с шансом 1/4 не умирает"])]
}

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Die { chr_id: _chr_id } if _chr_id == chr_id => {
            if game.input.random_bool(1. / 4.) {
                return Err(Cancelled("[БЕАТРИЧЕ]: random bool == true"));
            }
        }

        _ => {}
    }

    Ok(signed_event)
}
