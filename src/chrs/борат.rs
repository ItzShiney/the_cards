pub use crate::chr_uses::*;

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

pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Place { chr_id: _chr_id } if _chr_id == chr_id => {
            let chr_id = chr_id;

            if let Some(gained_act_id) = game.state.acts.pick(game.state.find_owner_of_chr(chr_id))
            {
                let could_use = Event::Use {
                    act_id: gained_act_id,
                    use_way: UseWay::OnCharacter(chr_id),
                }
                .sign(chr_id)
                .try_(game)
                .is_ok();

                if !could_use {
                    _ = Event::PutActiveInDrawpile {
                        act_id: gained_act_id,
                    }
                    .sign(chr_id)
                    .try_(game);
                }
            }
        }

        _ => {}
    }

    Ok(signed_event)
}
