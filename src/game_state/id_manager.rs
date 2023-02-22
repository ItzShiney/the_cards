use self::id_trait::IDTrait;

pub mod id_trait;

pub struct IDManager<T: IDTrait> {
    next_id: T,
}

impl<T: IDTrait> Default for IDManager<T> {
    fn default() -> Self {
        Self { next_id: T::FIRST }
    }
}

impl<T: IDTrait> IDManager<T> {
    pub fn next_id(&mut self) -> T {
        let res = self.next_id;
        self.next_id.next();
        res
    }
}
