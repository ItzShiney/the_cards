use crate::game::state::player_id::PlayerID;

pub struct ChooseCardArgs {
    pub is_cancellable: bool,
    pub player_id: PlayerID,
}
