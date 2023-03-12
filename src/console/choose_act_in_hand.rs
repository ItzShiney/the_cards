use super::prompt;
use crate::cs;
use crate::game::input::ChooseActInHand;
use crate::game::input::ChooseCardArgsP;
use crate::game::state::act_id::ActiveID;
use crate::game::state::GameState;
use itertools::Itertools;

pub struct ChooseActiveInHand;

impl ChooseActInHand for ChooseActiveInHand {
    fn choose_act_in_hand<'game_state>(
        &mut self,
        game_state: &'game_state GameState,
        args: ChooseCardArgsP<'_, 'game_state, '_, ActiveID>,
    ) -> Option<ActiveID> {
        let acts = game_state.acts.hand(args.player_id).clone();
        let is_enabled =
            acts.iter().copied().map(|act_id| (args.p)(game_state, act_id)).collect_vec();

        let is_any_enabled =
            is_enabled.iter().copied().zip(acts.iter().copied()).any(|(is_enabled, _)| is_enabled);

        if !is_any_enabled {
            return None;
        }

        let displays =
            acts.clone().into_iter().map(|act_id| cs![Active(game_state.act(act_id).type_)]);

        let act_idx =
            prompt(args.prompt, args.is_cancellable, is_enabled.iter().copied().zip(displays))?;
        Some(acts[act_idx])
    }
}
