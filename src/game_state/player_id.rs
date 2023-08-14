use super::id_manager::id_trait::IDTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlayerID(pub usize);

impl IDTrait for PlayerID {
    const FIRST: Self = Self(1);

    fn next(&mut self) {
        self.0 += 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerOwned<T> {
    pub owner_id: PlayerID,
    pub value: T,
}
