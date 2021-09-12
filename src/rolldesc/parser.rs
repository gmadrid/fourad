use crate::rolldesc::{RollDesc, RollModifier};
use crate::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

// TODO: write a grammar and a parser for the dice codes. Regex won't allow good error reporting.

lazy_static! {
    static ref RE: Regex = Regex::new("([[:digit:]]*)d([[:digit:]])([+-]([[:digit:]]+))?").unwrap();
}

const REPEAT_MATCH: usize = 1;
const MODIFIER_MATCH: usize = 3;
const MODIFIER_OPERAND_MATCH: usize = 4;
const SIDES_MATCH: usize = 2;

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
    s.parse::<u8>()
        .map_err(|err| Error::BadSidesString(s.to_string(), err))
}

pub fn parse_diecode(s: &str) -> Result<RollDesc> {
    // TODO: do you want to allow spaces
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
        let repeat =
            parse_repeat(caps.get(REPEAT_MATCH).map(|m| m.as_str()).unwrap_or("1")).unwrap();
        // unwrap: the regex will not match unless there is a value here.
        let sides = parse_sides(caps.get(SIDES_MATCH).map(|m| m.as_str()).unwrap())?;
        let modifier = parse_modifier(
            caps.get(MODIFIER_MATCH).map(|m| m.as_str()).unwrap_or(""),
            caps.get(MODIFIER_OPERAND_MATCH)
                .map(|m| m.as_str())
                .unwrap_or(""),
        );
        Ok(RollDesc {
            repeat,
            modifier,
            sides,
        })
    } else {
        Err(Error::UnknownError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_d6() {
        assert_eq!(parse_diecode("d6").unwrap(), RollDesc::default());
    }

    #[test]
    fn repeat_2d6() {
        assert_eq!(
            parse_diecode("2d6").unwrap(),
            RollDesc {
                repeat: 2,
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn modifier_d6plus1() {
        assert_eq!(
            parse_diecode("d6+1").unwrap(),
            RollDesc {
                modifier: RollModifier::Plus(1),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn repeat_modifier_2d6plus1() {
        assert_eq!(
            parse_diecode("2d6+1").unwrap(),
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
            parse_diecode("2d6+2").unwrap(),
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
            parse_diecode("d6xd6").unwrap(),
            RollDesc {
                modifier: RollModifier::Squared,
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d6minus1() {
        assert_eq!(
            parse_diecode("d6-1").unwrap(),
            RollDesc {
                modifier: RollModifier::Minus(1),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d6minus2() {
        assert_eq!(
            parse_diecode("d6-2").unwrap(),
            RollDesc {
                modifier: RollModifier::Minus(2),
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn hundo_d66() {
        assert_eq!(
            parse_diecode("d66").unwrap(),
            RollDesc {
                modifier: RollModifier::Hundo,
                ..RollDesc::default()
            }
        )
    }

    #[test]
    fn test_d3() {
        assert_eq!(
            parse_diecode("d3").unwrap(),
            RollDesc {
                sides: 3,
                ..RollDesc::default()
            }
        )
    }
}
