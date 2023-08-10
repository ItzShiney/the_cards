use crate::game::Game;

pub struct GameFormatted<'game, 'state, 'input, T> {
    pub game: &'game Game<'state, 'input>,
    pub value: T,
}

impl<'game, 'state, 'input, A> GameFormatted<'game, 'state, 'input, A> {
    pub fn with_value<T>(&self, value: T) -> GameFormatted<'game, 'state, 'input, T> {
        let Self { game, .. } = self;

        GameFormatted { game, value }
    }
}
