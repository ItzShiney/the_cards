use super::prompt;
use crate::cs;
use crate::game::input::ChooseCardArgsP;
use crate::game::input::ChooseChrInHand;
use crate::game::state::chr_id::CharacterID;
use crate::game::state::GameState;
use itertools::Itertools;

pub struct ChooseCharacterInHand;

impl ChooseChrInHand for ChooseCharacterInHand {
    fn choose_chr_in_hand<'game_state>(
        &mut self,
        game_state: &'game_state GameState,
        args: ChooseCardArgsP<'_, 'game_state, '_, CharacterID>,
    ) -> Option<CharacterID> {
        let chrs = game_state.chrs.hand(args.player_id).clone();
        let is_enabled =
            chrs.iter().copied().map(|chr_id| (args.p)(game_state, chr_id)).collect_vec();

        let is_any_enabled =
            is_enabled.iter().copied().zip(chrs.iter().copied()).any(|(is_enabled, _)| is_enabled);

        if !is_any_enabled {
            return None;
        }

        let displays =
            chrs.clone().into_iter().map(|chr_id| cs![Character(game_state.chr(chr_id).type_)]);

        let chr_idx =
            prompt(args.prompt, args.is_cancellable, is_enabled.iter().copied().zip(displays))?;
        Some(chrs[chr_idx])
    }
}
