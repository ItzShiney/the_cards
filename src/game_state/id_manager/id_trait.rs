use std::hash::Hash;

pub trait IDTrait: Hash + Eq + Copy {
    const FIRST: Self;

    fn next(&mut self);
}
