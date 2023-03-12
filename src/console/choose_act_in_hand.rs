use super::prompt;
use crate::cs;
use crate::game::input::ChooseActInHand;
use crate::game::input::ChooseCardArgsP;
use crate::game::state::act_id::ActiveID;
use crate::game::state::GameState;
use itertools::Itertools;

pub struct ChooseActiveInHand;

impl ChooseActInHand for ChooseActiveInHand {
    fn choose_act_in_hand<'prompt_str, 'game_state>(
        &mut self,
        game_state: &'game_state GameState,
        args: ChooseCardArgsP<'prompt_str, 'game_state, '_, ActiveID>,
    ) -> Option<ActiveID> {
        let acts = game_state
            .acts
            .hand(args.player_id)
            .iter()
            .copied()
            .filter(|&act_id| (args.p)(game_state, act_id))
            .collect_vec();

        if acts.is_empty() {
            return None;
        }

        let displays =
            acts.clone().into_iter().map(|act_id| cs![Active(game_state.act(act_id).type_)]);
        let results = acts.iter().copied();
        prompt(args.prompt, args.is_cancellable, displays, results)
    }
}
