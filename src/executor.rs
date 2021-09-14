use crate::grammar::{DieCode, Factor, Modifier};
use crate::roller::RandRoller;
use crate::roller::Roller;
use std::ops::ControlFlow;

pub fn execute(code: DieCode, explode: bool, force_66: bool) -> i16 {
    execute_with_roller(code, explode, force_66, &mut RandRoller::default())
}

pub fn execute_with_roller<R>(code: DieCode, explode: bool, force_66: bool, roller: &mut R) -> i16
where
    R: Roller,
{
    Executor { code, explode, force_66 }.execute(roller)
}

struct Executor {
    code: DieCode,
    explode: bool,
    force_66: bool,
}

impl Executor {
    fn execute(&self, roller: &mut impl Roller) -> i16 {
        self.code
            .factors
            .iter()
            .map(|f| self.execute_factor(f, roller))
            .product()
    }

    fn execute_factor(&self, factor: &Factor, roller: &mut impl Roller) -> i16 {
        let total = (0..factor.repeat.number)
            .map(|_| self.roll(factor.sides, self.explode(), roller))
            .sum();

        self.modify(&factor.modifier, total)
    }

    fn roll(&self, sides: u8, explode: bool, roller: &mut impl Roller) -> i16 {
        if sides == 66 && !self.force_66 {
            // special case!
            return self.roll_d66(roller);
        }

        match std::iter::repeat_with(|| roller.roll(sides))
            // TODO: add a quiet option
            .inspect(|die| println!("Rolled: {}", die))
            .try_fold(0, |sum, die| {
                if die != 6 || !explode {
                    ControlFlow::Break(sum + die)
                } else {
                    ControlFlow::Continue(sum + die)
                }
            }) {
            ControlFlow::Break(sum) => sum as i16,
            _ => panic!("This shouldn't happen"),
        }
    }

    fn roll_d66(&self, roller: &mut impl Roller) -> i16 {
        // d66 *never* explodes.
        self.roll(6, false, roller) * 10 + self.roll(6, false, roller)
    }

    fn modify(&self, modifier: &Modifier, unmodified: i16) -> i16 {
        unmodified
            + match modifier {
                Modifier::None => 0i16,
                Modifier::Plus(operand) => *operand as i16,
                Modifier::Minus(operand) => -(*operand as i16),
            }
    }

    fn explode(&self) -> bool {
        self.explode || self.code.directives.explode
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::roller::iterroller::IterRoller;

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
