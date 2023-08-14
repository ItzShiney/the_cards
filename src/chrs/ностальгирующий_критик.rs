pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["НОСТАЛЬГИРУЮЩИЙ КРИТИК"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByКостя,
        genders: [Мужчина],
        tags: [],
    }.into()
}

// 4/3/-2
#[rustfmt::skip]
pub fn stats() -> Stats {
    Stats::new(
        phy!(7),
        dmg!(6),
        int!(6),
    )
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["пока ", Intellect, " противника ", LE, " 3"]),
        Point(cs![
            Vitality,
            " этой карты на 1 меньше, ",
            Damage,
            " на 2 больше"
        ]),
    ]
}

pub fn handle_check(
    game: &Game,
    chr_id: CharacterID,
    mut signed_check: SignedCheck,
) -> CheckResult {
    match &mut signed_check.value {
        &mut Check::Stat {
            chr_id: _chr_id,
            stat_type,
            ref mut value,
        } if _chr_id == chr_id => {
            let enemy_id = game
                .state
                .chrs_on_field(
                    game.state.turn_info.id_by_subturner(
                        game.state
                            .turn_info
                            .subturner_by_id(
                                game.state
                                    .try_owner_id(chr_id)
                                    .ok_or(Cancelled("no owner"))?,
                            )
                            .ok_or(Cancelled("else's turn"))?
                            .other(),
                    ),
                )
                .next()
                .ok_or(Cancelled("no placed chr"))?;
            let enemy_int = game.stat(enemy_id, stat_type, chr_id);

            if enemy_int <= 3 {
                match stat_type {
                    StatType::Vitality => *value -= 1,
                    StatType::Damage => *value += 1,
                    _ => {}
                }
            }
        }

        _ => {}
    }

    Ok(signed_check)
}
