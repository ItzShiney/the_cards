pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["БЕРН"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByМаксим,
        genders: [],
        tags: [Umineko],
    }
    .into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована на противника, единственного на поле"]),
        Point(cs!["противник обязан поменять персонажа"]),
    ]
}

pub fn use_on_chr(
    game: &mut Game,
    act_id: ActiveID,
    chr_id: CharacterID,
) -> Result<CharacterID, Cancelled> {
    let target_owner_id = game.state.owner_id(chr_id);

    let replacing_chr_id = game
        .choose_chr_in_hand(ChooseCardArgsP {
            prompt: PromptArgs {
                str: cs![Active(Берн), ": на кого поменять?"],
                is_cancellable: false,
                autochoose_single_option: true,
            },
            player_id: target_owner_id,
            p: &|game, prompt_chr_id| {
                prompt_chr_id != chr_id && game.can(Event::Place { chr_id }.sign(act_id))
            },
        })
        .unwrap();

    Event::Replace {
        replaced_chr_id: chr_id,
        replacing_chr_id,
    }
    .sign(act_id)
    .try_(game)?;

    Ok(chr_id)
}
