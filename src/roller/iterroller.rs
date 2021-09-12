use crate::roller::Roller;

pub struct IterRoller<I>
where
    I: Iterator<Item = u8>,
{
    i: I,
}

impl<I> IterRoller<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(i: I) -> IterRoller<I> {
        IterRoller { i }
    }
}

impl<I> Roller for IterRoller<I>
where
    I: Iterator<Item = u8>,
{
    fn roll(&mut self, sides: u8) -> u8 {
        // unwrap: this is for testing. The creator should ensure enough values for their use case.
        self.i.next().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::repeat;

    #[test]
    fn test_vec() {
        let vec = vec![2u8, 3, 5, 4];

        let mut r = IterRoller::new(vec.into_iter());
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 3);
        assert_eq!(r.roll(6), 5);
        assert_eq!(r.roll(6), 4);
    }

    #[test]
    fn test_repeat() {
        let mut r = IterRoller::new(repeat(2));
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 2);
        assert_eq!(r.roll(6), 2);
    }
}
