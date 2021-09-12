use crate::rolldesc::{RollDesc, RollModifier};
use crate::roller::RandRoller;
use crate::roller::Roller;

pub fn execute(desc: RollDesc, explodes: bool) -> i16 {
    execute_with_roller(desc, explodes, &mut RandRoller::default())
}

fn execute_with_roller<R>(desc: RollDesc, explodes: bool, roller: &mut R) -> i16 where R: Roller {
    Executor { explodes, desc }.execute(roller)
}

struct Executor {
    explodes: bool,
    desc: RollDesc,
}

fn roll(explode: bool, roller: &mut impl Roller, sides: u8) -> u8 {
    let mut sum = 0;
    let mut done = false;

    while !done {
        let die = roller.roll(sides);
        sum += die;

        // TODO: add a quiet option
        println!("Rolled: {}", die);

        if die != 6 || explode == false {
            done = true
        }
    }

    sum
}

fn rolls(repeat: u8, roller: &mut impl Roller, sides: u8) -> i16 {
    (0..repeat).map(|_| roll(true, roller, sides) as i16).sum()
}

impl Executor {
    fn execute<R>(&self, roller: &mut R) -> i16 where R: Roller {
        match self.desc.modifier {
            RollModifier::None => rolls(self.desc.repeat, roller, self.desc.sides),
            RollModifier::Plus(val) => rolls(self.desc.repeat, roller, self.desc.sides) + val as i16,
            RollModifier::Minus(val) => rolls(self.desc.repeat, roller, self.desc.sides) - val as i16,
            RollModifier::Squared => {
                (roll(false, roller, self.desc.sides) * roll(false, roller, self.desc.sides)) as i16
            }
            RollModifier::Hundo => {
                (roll(false, roller, self.desc.sides) * 10 + roll(false, roller, self.desc.sides)) as i16
            }
        }
    }
}
