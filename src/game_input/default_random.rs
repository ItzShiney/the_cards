use super::Random;
use crate::stats::Stat0;
use rand::thread_rng;
use rand::Rng;

pub struct DefaultRandom;

impl Random for DefaultRandom {
    fn random(&mut self, min: Stat0, max: Stat0) -> Stat0 {
        thread_rng().gen_range(min..=max)
    }
}
