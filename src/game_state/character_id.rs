use super::id_manager::id_trait::IDTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CharacterID(usize);

impl IDTrait for CharacterID {
    const FIRST: Self = Self(1);

    fn next(&mut self) {
        self.0 += 1
    }
}
