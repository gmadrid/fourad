// Need to match:
//   d6    X
//   d6+1  X
//  2d6    X
//  2d6+1  X
//  2d6+2  X
//   d6xd6 X
//   d6-1  X
//   d6-2  X
//   d66   X

//
// <MULT:int>'d|D'6<OP><OPERAND>
//
// Regex:   [0-9]*[dD]6([+-xX][0-9L]
// "([[:digit:]]*)d6([+-]([[:digit:]]+))?"
// match 1 => repeat
// match 2 => modifier part
// match 3 => modifier value

// TODO: write a grammar and a parser for the dice codes. Regex won't allow good error reporting.

use fourad::{Error, Result};

use lazy_static::lazy_static;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref RE: Regex = Regex::new("([[:digit:]]*)d([[:digit:]])([+-]([[:digit:]]+))?").unwrap();
}

const REPEAT_MATCH: usize = 1;
const MODIFIER_MATCH: usize = 3;
const MODIFIER_OPERAND_MATCH: usize = 4;
const SIDES_MATCH: usize = 2;

#[derive(Debug, Eq, PartialEq)]
struct RollDesc {
    repeat: u8,
    sides: u8,
    modifier: RollModifier,
}

impl Default for RollDesc {
    fn default() -> Self {
        RollDesc {
            repeat: 1,
            sides: 6,
            modifier: RollModifier::default(),
        }
    }
}

impl RollDesc {
    fn execute(&self, roller: &mut impl Roller) -> i16 {
        match self.modifier {
            RollModifier::None => rolls(self.repeat, roller),
            RollModifier::Plus(val) => rolls(self.repeat, roller) + val as i16,
            RollModifier::Minus(val) => rolls(self.repeat, roller) - val as i16,
            RollModifier::Squared => (roll(false, roller) * roll(false, roller)) as i16,
            RollModifier::Hundo => (roll(false, roller) * 10 + roll(false, roller)) as i16,
        }
    }
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

fn roll(explode: bool, roller: &mut impl Roller) -> u8 {
    let mut sum = 0;
    let mut done = false;

    while !done {
        let die = roller.roll(6);
        sum += die;

        // TODO: add a quiet option
        println!("Rolled: {}", die);

        if die != 6 || explode == false {
            done = true
        }
    }

    sum
}

fn rolls(repeat: u8, roller: &mut impl Roller) -> i16 {
    (0..repeat).map(|_| roll(true, roller) as i16).sum()
}

fn parse_repeat(s: &str) -> Result<u8> {
    if s.is_empty() {
        Ok(1)
    } else {
        s.parse()
            .map_err(|err| Error::ParseRepeatError(s.to_string(), err))
    }
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

fn parse_sides(s: &str) -> Result<u8> {
    s.parse::<u8>().map_err(|err|Error::BadSidesString(s.to_string(), err))
}

impl FromStr for RollDesc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // What a cheaty special case.
        if s.starts_with("d6xd6") {
            return Ok(RollDesc {
                modifier: RollModifier::Squared,
                ..RollDesc::default()
            });
        }

        // What a cheaty special case.
        if s.starts_with("d66") {
            return Ok(RollDesc {
                modifier: RollModifier::Hundo,
                ..RollDesc::default()
            });
        }

        if let Some(caps) = RE.captures(s) {
            // TODO: fix error checking
            let repeat = parse_repeat(caps.get(REPEAT_MATCH).map(|m| m.as_str()).unwrap_or("1")).unwrap();
            // unwrap: the regex will not match unless there is a value here.
            let sides = parse_sides(caps.get(SIDES_MATCH).map(|m| m.as_str()).unwrap())?;
            let modifier = parse_modifier(
                caps.get(MODIFIER_MATCH).map(|m| m.as_str()).unwrap_or(""),
                caps.get(MODIFIER_OPERAND_MATCH).map(|m| m.as_str()).unwrap_or(""),
            );
            Ok(RollDesc { repeat, modifier, sides, })
        } else {
            Err(Error::UnknownError)
        }
    }
}

trait Roller {
    fn roll(&mut self, sides: u8) -> u8;
}

struct RandRoller {
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

fn main() -> Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "d6".to_string());
    let desc: RollDesc = arg.parse()?;

    println!("{}", desc.execute(&mut RandRoller::default()));
    Ok(())
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

    #[test]
    fn test_d3() {
        assert_eq!(
            "d3".parse::<RollDesc>().unwrap(),
            RollDesc {
                sides: 3,
                ..RollDesc::default()
            }
        )
    }

    // TODO: test execute()
}
