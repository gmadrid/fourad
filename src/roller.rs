use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

pub trait Roller {
    fn roll(&mut self, sides: u8) -> u8;
}

pub struct RandRoller {
    rng: ThreadRng,
}

impl Default for RandRoller {
    fn default() -> Self {
        RandRoller { rng: thread_rng() }
    }
}

impl Roller for RandRoller {
    fn roll(&mut self, sides: u8) -> u8 {
        self.rng.gen_range(1..=sides)
    }
}
