use crate::stats::Stat0;
use rand::thread_rng;
use rand::Rng;

pub struct DefaultRandom;

impl DefaultRandom {
    pub fn random(&mut self, min: Stat0, max: Stat0) -> Stat0 {
        thread_rng().gen_range(min..=max)
    }

    pub fn random_bool(&mut self, true_prob: f64) -> bool {
        thread_rng().gen_bool(true_prob)
    }
}
