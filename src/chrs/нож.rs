pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["НОЖ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: D,
        author: ByЛёня,
        genders: [],
        tags: [TBoI, Нераздаваемая],
    }.into()
}

// 2/?/-0
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(3),
        dmg!(5?),
        int!(1),
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["выставлен"]),
        Point(cs![
            Damage,
            " = ",
            Sum {
                times: cs!["9"],
                body: cs![Random(cs!["0"]..=cs!["1"])]
            }
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
            let value = repeat_with(|| game.random(0, 1, chr_id)).take(9).sum();

            let chr_id = chr_id;
            Event::stat_change(chr_id, StatType::Damage, StatChange::Set(value));
        }

        _ => {}
    }

    Ok(signed_event)
}
