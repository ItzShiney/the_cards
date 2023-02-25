use crate::game_state::active_id::ActiveID;

use super::{active_trigger::ActiveTrigger, Ability};

pub type ActiveAbility = Ability<ActiveTrigger, ActiveID>;
