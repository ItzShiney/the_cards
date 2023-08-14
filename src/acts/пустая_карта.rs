pub use crate::act_uses::*;

pub fn name() -> CustomString {
    cs!["ПУСТАЯ КАРТА"]
}

#[rustfmt::skip]
pub fn groups() -> Groups {
    GroupsBuilder {
        tier: C,
        author: ByМаксим,
        genders: [],
        tags: [TBoI, Иллюзия],
    }.into()
}

pub fn description() -> CustomString {
    cs![
        Condition(cs!["использована"]),
        Point(cs![
            "выбери активку в руке. эта карта повторит эффект выбранной"
        ]),
    ]
}

pub fn use_on_field(game: &mut Game, act_id: ActiveID) -> Result<(), Cancelled> {
    let owner_id = game.state.owner_id(act_id);
    let imitated_act_id = game
        .choose_act_in_hand(ChooseCardArgsP {
            prompt: PromptArgs {
                str: cs![Active(ПустаяКарта), ": чей эффект повторить?"],
                is_cancellable: false,
                autochoose_single_option: false,
            },
            player_id: owner_id,
            p: &|game, act_id| act_id != act_id && game.can_use_in_any_way(act_id),
        })
        .unwrap();

    todo!("повторить эффект {:?}", imitated_act_id);
}
