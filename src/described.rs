use crate::game_state::ability_description::AbilityDescription;

pub struct Described<T> {
    pub description: AbilityDescription,
    pub value: T,
}
