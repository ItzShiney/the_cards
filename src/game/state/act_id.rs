use super::id_manager::id_trait::IDTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActiveID(usize);

impl IDTrait for ActiveID {
    const FIRST: Self = Self(1);

    fn next(&mut self) {
        self.0 += 1
    }
}
