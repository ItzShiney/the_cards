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
        args: ChooseCardArgsP<'game_state, '_, CharacterID>,
    ) -> Option<CharacterID> {
        let chrs = game_state
            .chrs
            .hand(args.player_id)
            .iter()
            .copied()
            .filter(|&chr_id| (args.p)(game_state, chr_id))
            .collect_vec();

        if chrs.is_empty() {
            return None;
        }

        let displays =
            chrs.clone().into_iter().map(|chr_id| cs![Character(game_state.chr(chr_id).type_)]);
        let results = chrs.iter().copied();
        prompt(args.is_cancellable, displays, results)
    }
}
