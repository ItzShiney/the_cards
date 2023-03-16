pub use crate::card_uses::*;

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
        // TODO
        Condition(cs!["пока ", Intellect, " противника ", LE, " 3"]),
        Point(cs![Vitality, " этой карты на 1 меньше, ", Damage, " на 2 больше"]),
    ]
}

pub fn abilities() -> GameCallbacks {
    GameCallbacks {
        stat_map: Some(|game, mut args| {
            let owner_id = game.state().find_owner_chr(args.chr_id);
            let Some(enemy_id) = game.state().try_enemy_chr_id(owner_id) else { return Continue(args) };
            let enemy_int = game.state().chr(enemy_id).stats.int.0.into_value();

            if enemy_int <= 3 {
                match args.stat_type {
                    StatType::Vitality => args.val -= 1,
                    StatType::Damage => args.val += 2,
                    _ => {}
                }
            }

            Continue(args)
        }),

        ..Default::default()
    }
}
