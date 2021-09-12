//
// Need to match:
//   d6    X
//   d6+1  X
//  2d6    X
//  2d6+1  X
//  2d6+2  X
//   d6xd6 X
//   d6-1  X
//   d6-2  X
//   d6+L
//   d6+hL
//   d6-L
//   d66   X

//
// <MULT:int>'d|D'6<OP><OPERAND>
//
// Regex:   [0-9]*[dD]6([+-xX][0-9L]

use rand::{thread_rng, Rng};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct RollDesc {
    repeat: u8,
    modifier: RollModifier,
}

#[derive(Debug, Eq, PartialEq)]
enum RollModifier {
    None,
    Plus(u8),
    Minus(u8),
    Squared, // d6xd6 (special case)
    Hundo,   // d66 (special case)
}

impl Default for RollModifier {
    fn default() -> Self {
        RollModifier::None
    }
}

impl Default for RollDesc {
    fn default() -> Self {
        RollDesc {
            repeat: 1,
            modifier: RollModifier::default(),
        }
    }
}

fn roll() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(1..=6)
}

fn rolls(repeat: u8) -> i16 {
    // TODO: rewrite this functionally.
    let mut sum: i16 = 0;
    for _ in 0..repeat {
        sum += roll() as i16;
    }

    sum
}

impl RollDesc {
    fn execute(&self) -> i16 {
        match self.modifier {
            RollModifier::None => rolls(self.repeat),
            RollModifier::Plus(val) => rolls(self.repeat) + val as i16,
            RollModifier::Minus(val) => rolls(self.repeat) - val as i16,
            RollModifier::Squared => (roll() * roll()) as i16,
            RollModifier::Hundo => (roll() * 10 + roll()) as i16,
        }
    }
}

fn parse_repeat(s: &str) -> u8 {
    s.parse::<u8>().unwrap_or(1)
}

fn parse_modifier(op: &str, value: &str) -> RollModifier {
    // TODO: better error checking here
    if op.is_empty() || value.is_empty() {
        RollModifier::None
    } else {
        let parsed_val = value.parse::<u8>().unwrap_or(1);
        // TODO: better error checking here.
        match op.chars().next() {
            Some('+') => RollModifier::Plus(parsed_val),
            Some('-') => RollModifier::Minus(parsed_val),
            _ => RollModifier::default(),
        }
    }
}

impl FromStr for RollDesc {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // What a cheaty special case.
        if s.starts_with("d6xd6") {
            return Ok(RollDesc {
                modifier: RollModifier::Squared,
                ..RollDesc::default()
            });
        }
        if s.starts_with("d66") {
            return Ok(RollDesc {
                modifier: RollModifier::Hundo,
                ..RollDesc::default()
            });
        }
        // TODO: make this lazy static?
        let re = Regex::new("([[:digit:]]*)d6([+-]([[:digit:]]+))?").unwrap();
        if let Some(caps) = re.captures(s) {
            let repeat = parse_repeat(caps.get(1).map(|m| m.as_str()).unwrap_or("1"));
            let modifier = parse_modifier(
                caps.get(2).map(|m| m.as_str()).unwrap_or(""),
                caps.get(3).map(|m| m.as_str()).unwrap_or(""),
            );
            Ok(RollDesc { repeat, modifier })
        } else {
            Err(())
        }
    }
}

fn main() {
    // TODO: better error checking
    let arg = std::env::args().nth(1).unwrap_or_else(|| "d6".to_string());
    let desc: RollDesc = arg.parse().unwrap_or_default();
    println!("{}", desc.execute());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_d6() {
        assert_eq!("d6".parse::<RollDesc>().unwrap(), RollDesc::default());
    }

    #[test]
    fn repeat_2d6() {
        assert_eq!(
            "2d6".parse::<RollDesc>().unwrap(),
            RollDesc {
                repeat: 2,
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn modifier_d6plus1() {
        assert_eq!(
            "d6+1".parse::<RollDesc>().unwrap(),
            RollDesc {
                modifier: RollModifier::Plus(1),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn repeat_modifier_2d6plus1() {
        assert_eq!(
            "2d6+1".parse::<RollDesc>().unwrap(),
            RollDesc {
                repeat: 2,
                modifier: RollModifier::Plus(1),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_2d6plus2() {
        assert_eq!(
            "2d6+2".parse::<RollDesc>().unwrap(),
            RollDesc {
                repeat: 2,
                modifier: RollModifier::Plus(2),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d6xd6() {
        assert_eq!(
            "d6xd6".parse::<RollDesc>().unwrap(),
            RollDesc {
                modifier: RollModifier::Squared,
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d6minus1() {
        assert_eq!(
            "d6-1".parse::<RollDesc>().unwrap(),
            RollDesc {
                modifier: RollModifier::Minus(1),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d6minus2() {
        assert_eq!(
            "d6-2".parse::<RollDesc>().unwrap(),
            RollDesc {
                modifier: RollModifier::Minus(2),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn hundo_d66() {
        assert_eq!(
            "d66".parse::<RollDesc>().unwrap(),
            RollDesc {
                modifier: RollModifier::Hundo,
                ..RollDesc::default()
            }
        )
    }

    // TODO: test execute()
}
