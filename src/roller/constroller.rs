use crate::roller::Roller;
use std::cmp::min;

struct ConstRoller {
    value: u8,
}

impl ConstRoller {
    pub fn new(value: u8) -> ConstRoller {
        ConstRoller { value }
    }
}

impl Roller for ConstRoller {
    fn roll(&mut self, sides: u8) -> u8 {
        min(self.value, sides)
    }
}

#[cfg(test)]
mod test {
    use crate::roller::constroller::ConstRoller;
    use crate::roller::Roller;

    #[test]
    fn base() {
        let mut r = ConstRoller::new(3);
        assert_eq!(r.roll(6), 3);
        assert_eq!(r.roll(6), 3);
        assert_eq!(r.roll(6), 3);
        assert_eq!(r.roll(6), 3);
        assert_eq!(r.roll(6), 3);
    }

    #[test]
    fn value_too_big() {
        let mut r = ConstRoller::new(12);
        assert_eq!(r.roll(6), 6);
        assert_eq!(r.roll(6), 6);
        assert_eq!(r.roll(6), 6);
        assert_eq!(r.roll(6), 6);
        assert_eq!(r.roll(6), 6);
    }
}
