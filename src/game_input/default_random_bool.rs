use super::RandomBool;
use rand::thread_rng;
use rand::Rng;

pub struct DefaultRandomBool;

impl RandomBool for DefaultRandomBool {
    fn random_bool(&mut self, true_prob: f64) -> bool {
        thread_rng().gen_bool(true_prob)
    }
}
