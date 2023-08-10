pub use crate::chr_uses::*;

pub fn name() -> CustomString {
    cs!["ДУХ ТВОЕЙ КВАРТИРЫ"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: B,
        author: ByКостя,
        genders: [Женщина],
        tags: [],
    }.into()
}

#[rustfmt::skip]
pub fn stats() -> Stats {
    // 4/3/-4
    Stats::new(
        phy!(8),
        dmg!(5),
        int!(1),
    )
}

pub fn description() -> CustomString {
    cs![
        Epitaph(cs!["\"твоё личное бревно\""]),
        __,
        Condition(cs!["пока у владельца ", LE, " 2 персонажей"]),
        Point(cs![Damage, " больше на 2"]),
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
        } if _chr_id == chr_id && stat_type == StatType::Damage => {
            let owner_id = game.state.find_owner_of_chr(chr_id);

            if game.state.chrs.hand(owner_id).len() <= 2 {
                *value += 2;
            }
        }

        _ => {}
    }

    Ok(signed_check)
}
