use crate::rolldesc::{RollDesc, RollModifier};
use crate::roller::RandRoller;
use crate::roller::Roller;

pub fn execute(desc: RollDesc, explodes: bool) -> i16 {
    execute_with_roller(desc, explodes, &mut RandRoller::default())
}

fn execute_with_roller<R>(desc: RollDesc, explodes: bool, roller: &mut R) -> i16
where
    R: Roller,
{
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

        if die != 6 || !explode {
            done = true
        }
    }

    sum
}

fn rolls(repeat: u8, roller: &mut impl Roller, sides: u8) -> i16 {
    (0..repeat).map(|_| roll(true, roller, sides) as i16).sum()
}

impl Executor {
    fn execute<R>(&self, roller: &mut R) -> i16
    where
        R: Roller,
    {
        match self.desc.modifier {
            RollModifier::None => rolls(self.desc.repeat, roller, self.desc.sides),
            RollModifier::Plus(val) => {
                rolls(self.desc.repeat, roller, self.desc.sides) + val as i16
            }
            RollModifier::Minus(val) => {
                rolls(self.desc.repeat, roller, self.desc.sides) - val as i16
            }
            RollModifier::Squared => {
                (roll(false, roller, self.desc.sides) * roll(false, roller, self.desc.sides)) as i16
            }
            RollModifier::Hundo => {
                (roll(false, roller, self.desc.sides) * 10 + roll(false, roller, self.desc.sides))
                    as i16
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::roller::iterroller::IterRoller;

    // TODO: test exploding

    #[test]
    fn test_basic() {
        let mut roller = IterRoller::new(vec![3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6".parse().unwrap(), true, &mut roller),
            3
        );
    }

    #[test]
    fn test_basic_exploding() {
        let mut roller = IterRoller::new(vec![6, 6, 3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6".parse().unwrap(), true, &mut roller),
            15
        );
    }

    #[test]
    fn test_two_exploding() {
        let mut roller = IterRoller::new(vec![6, 6, 3, 6, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("2d6".parse().unwrap(), true, &mut roller),
            26
        );
    }

    #[test]
    fn test_plusmod() {
        let mut roller = IterRoller::new(vec![3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6+4".parse().unwrap(), true, &mut roller),
            7
        );
    }

    #[test]
    fn test_minusmod() {
        let mut roller = IterRoller::new(vec![3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6-2".parse().unwrap(), true, &mut roller),
            1
        );
    }

    #[test]
    fn test_negative_result() {
        let mut roller = IterRoller::new(vec![3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6-7".parse().unwrap(), true, &mut roller),
            -4
        );
    }

    #[test]
    fn test_d66() {
        let mut roller = IterRoller::new(vec![3, 4, 5, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d66".parse().unwrap(), false, &mut roller),
            34
        );
    }

    #[test]
    fn test_d6xd6() {
        let mut roller = IterRoller::new(vec![3, 6, 4, 1, 1, 1, 1, 1].into_iter());
        assert_eq!(
            execute_with_roller("d6xd6".parse().unwrap(), false, &mut roller),
            18
        );
    }
}
