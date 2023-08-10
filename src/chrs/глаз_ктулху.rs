pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ГЛАЗ КТУЛХУ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByМаксим,
        genders: [],
        tags: [Terraria],
    }.into()
}

// 4/3/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(8),
        dmg!(6),
        int!(2),
    )
}

pub fn description() -> CustomString {
    cs![NamedPoint(
        cs!["\"ТАРАНИТ... ИНОГДА\""],
        cs!["с шансом 1/2 наносит на 1 ", Damage, " больше"]
    ),]
}

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    mut signed_event: SignedEvent,
) -> EventResult {
    match &mut signed_event.value {
        &mut Event::Attack {
            attacker_id: _chr_id,
            target_id: _,
            ref mut dmg,
        } if _chr_id == chr_id => {
            let Event::RandomBool {
                output: Some(random_bool),
                ..
            } = game
                .try_(Event::random_bool(1. / 2.).sign(chr_id))?
                .value
            else {
                unreachable!()
            };

            if random_bool {
                *dmg += 1;
            }
        }

        _ => {}
    }

    Ok(signed_event)
}
