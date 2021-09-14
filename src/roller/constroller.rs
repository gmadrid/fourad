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
    fn roll(&mut self, _sides: u8) -> u8 {
        self.value
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
}
