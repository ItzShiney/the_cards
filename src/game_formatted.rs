use crate::game::Game;

pub struct GameFormatted<'game, 'state, 'input, T, ID: Copy> {
    pub value: T,
    pub game: &'game Game<'state, 'input>,
    pub id: ID,
}

impl<'game, 'state, 'input, T, ID: Copy> GameFormatted<'game, 'state, 'input, T, ID> {
    pub fn with_value<U>(&self, new_value: U) -> GameFormatted<'game, 'state, 'input, U, ID> {
        GameFormatted { value: new_value, game: self.game, id: self.id }
    }
}
