/*
   2d6+3  (all dice types)
   d6xd10
   d66, d88 (deal with ambiguity)

   d6E  (explodes!)

   // Arbitrary string of d6xd6xd6xd6
*/

use crate::{Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct DieCode {
    factors: Vec<Factor>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Factor {
    repeat: Repeat,
    sides: u8,
    modifier: Modifier,
    directives: Directives,
}

impl Default for Factor {
    // The default Factor is "d6"
    fn default() -> Self {
        Factor {
            repeat: Default::default(),
            sides: 6,
            modifier: Default::default(),
            directives: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Repeat {
    number: u8,
}

impl Default for Repeat {
    fn default() -> Self {
        Repeat { number: 1 }
    }
}

// #[derive(Debug, Eq, PartialEq)]
// pub struct Modifier {
//     op: Opcode,
// }
//
// impl Default for Modifier {
//     fn default() -> Self {
//         Modifier { op: Opcode::None }
//     }
// }

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
    explode: bool,
}

/*
  Arbitrary string of d6xd6xd6xd6

  diecode  --> factor codetail
*/
pub fn parse_diecode(s: &str) -> Result<DieCode> {
    let mut factors = vec![];

    let (factor, rest) = parse_factor(s)?;
    factors.push(factor);

    parse_codetail(rest, &mut factors)?;

    Ok(DieCode { factors })
}

/*
 codetail --> 'x' factor codetail
          -->
*/
fn parse_codetail(s: &str, factors: &mut Vec<Factor>) -> Result<()> {
    if let Some(ch) = s.chars().nth(0) {
        if ch == 'x' {
            let (factor, rest) = parse_factor(&s[1..])?;
            factors.push(factor);

            parse_codetail(rest, factors)?;
        }

        // Ignore any unexpected suffix.
        // TODO: is this the behavior that we want?
    }
    Ok(())
}

/*
   factor --> repeat 'd' sides modifier directives
*/
fn parse_factor(s: &str) -> Result<(Factor, &str)> {
    let (repeat, rest) = parse_repeat(s)?;

    if let Some(ch) = rest.chars().nth(0) {
        if ch != 'd' {
            return Err(Error::UnexpectedChar('d', rest.to_string()));
        }
    } else {
        return Err(Error::UnexpectedEndOfString(s.to_string()));
    }

    let (sides, rest) = parse_sides(&rest[1..])?;
    let (modifier, rest) = parse_modifier(rest)?;
    let (directives, rest) = parse_directives(rest)?;

    let factor = Factor {
        repeat: repeat,
        sides,
        modifier: modifier,
        directives: directives,
    };
    Ok((factor, rest))
}

/*
   repeat --> number
          -->
*/
fn parse_repeat(s: &str) -> Result<(Repeat, &str)> {
    if s.starts_with(|ch: char| ch.is_ascii_digit()) {
        // Find the first ch that is not a digit.
        // TODO: you could do this as a single call to find and skip the starts_with
        // TODO: should be using parse_number here.
        if let Some(end) = s.find(|ch: char| !ch.is_ascii_digit()) {
            let number = s[..end].parse::<u8>()?;
            if number == 0 {
                Err(Error::ZeroRepeats)
            } else {
                let repeat = Repeat { number };
                Ok((repeat, &s[end..]))
            }
        } else {
            // This is the case where we find the beginning of the
            // repeat number, but there is nothing after it.
            Err(Error::UnexpectedEndOfString(s.to_string()))
        }
    } else {
        // A missing Repeat is equivalent to a '1'.
        Ok((Repeat { number: 1 }, s))
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
  number --> [[:digit:]]+
*/
fn parse_number(s: &str) -> Result<(u8, &str)> {
    if let Some(end) = s.find(|ch: char| !ch.is_ascii_digit()) {
        Ok((s[..end].parse::<u8>()?, &s[end..]))
    } else {
        Ok((s.parse::<u8>()?, ""))
    }
}

/*
  modifier --> '+' operand
           --> '-' operand
           -->
*/
fn parse_modifier(s: &str) -> Result<(Modifier, &str)> {
    if s.starts_with('+') {
        let (operand, rest) = parse_operand(&s[1..])?;
        Ok((Modifier::Plus(operand), rest))
    } else if s.starts_with('-') {
        let (operand, rest) = parse_operand(&s[1..])?;
        Ok((Modifier::Minus(operand), rest))
    } else {
        Ok((Modifier::None, s))
    }
}

/*
  operand --> number
*/
fn parse_operand(s: &str) -> Result<(u8, &str)> {
    parse_number(s)
}

/*
 directives --> 'E'
            -->
*/
fn parse_directives(s: &str) -> Result<(Directives, &str)> {
    if s.starts_with('E') {
        let directives = Directives {
            explode: true,
            ..Directives::default()
        };
        Ok((directives, &s[1..]))
    } else {
        Ok((Directives::default(), s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: check that sides != 0.
    // TODO: check that repeat != 0.

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
                        directives: Directives { explode: true },
                        ..Factor::default()
                    },
                ]
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
                directives: Default::default()
            }
        );

        let (factor, rest) = parse_factor("3d12-3E").unwrap();
        assert_eq!(
            factor,
            Factor {
                repeat: Repeat { number: 3 },
                sides: 12,
                modifier: Modifier::Minus(3),
                directives: Directives { explode: true },
            }
        )
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
