pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ГВ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByМаксим,
        genders: [Мужчина, Женщина],
        tags: [Umineko],
    }.into()
}

// 0/5/-3
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(0?),
        dmg!(7),
        int!(7),
    )
}

pub fn description() -> CustomString {
    cs![
        // TODO
        Activatable,
        Point(cs![
            "этот персонаж превращается в выбранного из трёх случайных ",
            Umineko,
            "-персонажей"
        ]),
        __,
        Condition(cs!["выставлен"]),
        Point(cs![
            Physique,
            " = ",
            SumAll {
                body: cs![Physique, " всех ", Иллюзия, " в руке"]
            }
        ]),
        Point(cs!["считает ", Берн, " за персонажа с ", Physique, " 3"]),
    ]
}
pub fn handle_event(
    game: &mut Game,
    chr_id: CharacterID,
    signed_event: SignedEvent,
) -> EventResult {
    match signed_event.value {
        Event::Place { chr_id: _chr_id } if _chr_id == chr_id => {
            let owner_id = game.state.find_owner_of_chr(chr_id);

            let value = {
                let chrs_sum = game
                    .state
                    .chrs
                    .hand(owner_id)
                    .iter()
                    .copied()
                    .filter_map(|chr_id| {
                        let chr = game.state.chr(chr_id);
                        if chr.type_.groups().contains(&Иллюзия) {
                            Some(chr.stats.phy.0)
                        } else {
                            None
                        }
                    })
                    .sum::<Stat0>();

                let acts_sum = game
                    .state
                    .acts
                    .hand(owner_id)
                    .iter()
                    .copied()
                    .filter_map(|act_id| {
                        let act = game.state.act(act_id);
                        match act.type_ {
                            Берн => Some(3),
                            _ => None,
                        }
                    })
                    .sum::<Stat0>();

                chrs_sum + acts_sum
            };

            Event::stat_change(chr_id, StatType::Physique, StatChange::Set(value))
                .sign(chr_id)
                .try_(game)?;

            Event::stat_change(chr_id, StatType::Vitality, StatChange::Set(value))
                .sign(chr_id)
                .try_(game)?;
        }

        _ => {}
    }

    Ok(signed_event)
}
