/*
   2d6+3  (all dice types)
   d6xd10
   d66, d88 (deal with ambiguity)

   d6E  (explode!)

   // Arbitrary string of d6xd6xd6xd6
*/

use crate::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct DieCode {
    pub factors: Vec<Factor>,
    pub directives: Directives,
}

impl FromStr for DieCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_diecode(s)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Factor {
    pub repeat: Repeat,
    pub sides: u8,
    pub modifier: Modifier,
}

impl Default for Factor {
    // The default Factor is "d6"
    fn default() -> Self {
        Factor {
            repeat: Default::default(),
            sides: 6,
            modifier: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Repeat {
    pub number: u8,
}

impl Default for Repeat {
    fn default() -> Self {
        Repeat { number: 1 }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier {
    None,
    Plus(u8),
    Minus(u8),
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::None
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Directives {
    pub explode: bool,
}

/*
  Arbitrary string of d6xd6xd6xd6

  GRAMMAR: diecode    --> factor codetail directives
*/
pub fn parse_diecode(s: &str) -> Result<DieCode> {
    let s = s.trim();

    let mut factors = vec![];

    let (factor, rest) = parse_factor(s)?;
    factors.push(factor);

    let rest = parse_codetail(rest, &mut factors)?;

    let (directives, rest) = parse_directives(rest)?;

    if !rest.is_empty() {
        return Err(Error::UnexpectedEOL(rest.to_string()));
    }

    Ok(DieCode {
        factors,
        directives,
    })
}

/*
  GRAMMAR: codetail   --> 'x' factor codetail
  GRAMMAR:            -->
*/
fn parse_codetail<'a>(s: &'a str, factors: &mut Vec<Factor>) -> Result<&'a str> {
    if let Some(ch) = s.chars().next() {
        if ch == 'x' {
            let (factor, rest) = parse_factor(&s[1..])?;
            factors.push(factor);

            return parse_codetail(rest, factors);
        }
    }
    Ok(s)
}

/*
  GRAMMAR: factor     --> repeat 'd' sides modifier
*/
fn parse_factor(s: &str) -> Result<(Factor, &str)> {
    let (repeat, rest) = parse_repeat(s)?;

    if let Some(ch) = rest.chars().next() {
        if ch != 'd' {
            return Err(Error::UnexpectedChar('d', rest.to_string()));
        }
    } else {
        return Err(Error::UnexpectedEndOfString(s.to_string()));
    }

    let (sides, rest) = parse_sides(&rest[1..])?;
    let (modifier, rest) = parse_modifier(rest)?;

    let factor = Factor {
        repeat,
        sides,
        modifier,
    };
    Ok((factor, rest))
}

/*
  GRAMMAR: repeat     --> number
  GRAMMAR:            -->
*/
fn parse_repeat(s: &str) -> Result<(Repeat, &str)> {
    if !s.starts_with(|ch: char| ch.is_ascii_digit()) {
        // A missing Repeat is equivalent to a '1'.
        Ok((Repeat { number: 1 }, s))
    } else {
        parse_number(s).and_then(|(number, rest)| {
            if number == 0 {
                Err(Error::ZeroRepeats)
            } else {
                Ok((Repeat { number }, rest))
            }
        })
    }
}

fn parse_sides(s: &str) -> Result<(u8, &str)> {
    parse_number(s).and_then(|(sides, rest)| {
        if sides == 0 || sides == 1 {
            Err(Error::ZeroOrOneSide)
        } else {
            Ok((sides, rest))
        }
    })
}

/*
  GRAMMAR: number     --> [[:digit:]]+
*/
fn parse_number(s: &str) -> Result<(u8, &str)> {
    if let Some(end) = s.find(|ch: char| !ch.is_ascii_digit()) {
        Ok((s[..end].parse::<u8>()?, &s[end..]))
    } else {
        Ok((s.parse::<u8>()?, ""))
    }
}

/*
  GRAMMAR: modifier   --> '+' operand
  GRAMMAR:            --> '-' operand
  GRAMMAR:            -->
*/
fn parse_modifier(s: &str) -> Result<(Modifier, &str)> {
    if let Some(rest) = s.strip_prefix('+') {
        let (operand, rest) = parse_operand(rest)?;
        Ok((Modifier::Plus(operand), rest))
    } else if let Some(rest) = s.strip_prefix('-') {
        let (operand, rest) = parse_operand(rest)?;
        Ok((Modifier::Minus(operand), rest))
    } else {
        Ok((Modifier::None, s))
    }
}

/*
  GRAMMAR: operand    --> number
*/
fn parse_operand(s: &str) -> Result<(u8, &str)> {
    parse_number(s)
}

/*
  GRAMMAR: directives --> 'E'
  GRAMMAR:            -->
*/
fn parse_directives(s: &str) -> Result<(Directives, &str)> {
    if let Some(rest) = s.strip_prefix('E') {
        Ok((Directives { explode: true }, rest))
    } else {
        Ok((Directives::default(), s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_diecode() {
        let diecode = parse_diecode("d6").unwrap();
        assert_eq!(1, diecode.factors.len());

        let diecode = parse_diecode("d6xd6xd6").unwrap();
        assert_eq!(3, diecode.factors.len());
    }

    #[test]
    fn test_parse_diecode_deep() {
        let diecode = parse_diecode("d6x2d6+1xd3-2E").unwrap();
        assert_eq!(
            diecode,
            DieCode {
                factors: vec![
                    Factor::default(),
                    Factor {
                        repeat: Repeat { number: 2 },
                        modifier: Modifier::Plus(1),
                        ..Factor::default()
                    },
                    Factor {
                        sides: 3,
                        modifier: Modifier::Minus(2),
                        ..Factor::default()
                    },
                ],
                directives: Directives { explode: true },
            }
        )
    }

    #[test]
    fn test_parse_factor() {
        let (factor, rest) = parse_factor("d6").unwrap();
        assert_eq!(
            factor,
            Factor {
                repeat: Default::default(),
                sides: 6,
                modifier: Default::default(),
            }
        );
        assert_eq!(rest, "");

        let (factor, rest) = parse_factor("3d12-3E").unwrap();
        assert_eq!(
            factor,
            Factor {
                repeat: Repeat { number: 3 },
                sides: 12,
                modifier: Modifier::Minus(3),
            }
        );
        assert_eq!(rest, "E");
    }

    #[test]
    fn test_parse_repeat() {
        let (repeat, rest) = parse_repeat("8d6").unwrap();
        assert_eq!(repeat, Repeat { number: 8 });
        assert_eq!(rest, "d6");

        let (repeat, rest) = parse_repeat("d6").unwrap();
        assert_eq!(repeat, Repeat { number: 1 });
        assert_eq!(rest, "d6");

        let err = parse_repeat("0d6").unwrap_err();
        assert!(matches!(err, Error::ZeroRepeats));
    }

    #[test]
    fn test_parse_sides() {
        let (sides, rest) = parse_sides("3").unwrap();
        assert_eq!(sides, 3);
        assert_eq!(rest, "");

        let (sides, rest) = parse_sides("6+3").unwrap();
        assert_eq!(sides, 6);
        assert_eq!(rest, "+3");

        let err = parse_sides("0").unwrap_err();
        assert!(matches!(err, Error::ZeroOrOneSide));

        let err = parse_sides("1").unwrap_err();
        assert!(matches!(err, Error::ZeroOrOneSide));
    }

    #[test]
    fn test_parse_number() {
        let (number, rest) = parse_number("64REST").unwrap();
        assert_eq!(number, 64);
        assert_eq!(rest, "REST");

        let (number, rest) = parse_number("83").unwrap();
        assert_eq!(number, 83);
        assert_eq!(rest, "");

        let err = parse_number("888").unwrap_err();
        assert!(matches!(err, Error::ParseNumberError(_)));

        let err = parse_number("MISSING").unwrap_err();
        assert!(matches!(err, Error::ParseNumberError(_)));
    }

    #[test]
    fn test_parse_modifier() {
        let (modifier, rest) = parse_modifier("NONE").unwrap();
        assert_eq!(modifier, Modifier::None);
        assert_eq!(rest, "NONE");

        // PLUS, AT END
        let (modifier, rest) = parse_modifier("+38").unwrap();
        assert_eq!(modifier, Modifier::Plus(38));
        assert_eq!(rest, "");

        // MINUS, WITH REST
        let (modifier, rest) = parse_modifier("-83REST").unwrap();
        assert_eq!(modifier, Modifier::Minus(83));
        assert_eq!(rest, "REST");
    }

    #[test]
    fn test_parse_operand() {
        let (operand, rest) = parse_operand("123REST").unwrap();
        assert_eq!(operand, 123);
        assert_eq!(rest, "REST");

        // TOO BIG
        let err = parse_operand("1234REST").unwrap_err();
        assert!(matches!(err, Error::ParseNumberError(_)));

        // MISSING
        let err = parse_operand("MISSING").unwrap_err();
        assert!(matches!(err, Error::ParseNumberError(_)));
    }

    #[test]
    fn test_parse_directives() {
        let (directives, rest) = parse_directives("").unwrap();
        assert_eq!(directives, Directives::default());
        assert_eq!(rest, "");

        let (directives, rest) = parse_directives("EEE").unwrap();
        assert_eq!(
            directives,
            Directives {
                explode: true,
                ..Directives::default()
            }
        );
        assert_eq!(rest, "EE");
    }
}
