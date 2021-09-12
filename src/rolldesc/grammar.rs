/*
   2d6+3  (all dice types)
   d6xd10
   d66, d88 (deal with ambiguity)

   d6E  (explodes!)

   // Arbitrary string of d6xd6xd6xd6
*/

use crate::FourADError::UnexpectedEndOfString;
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

#[derive(Debug, Eq, PartialEq)]
pub struct Repeat {
    number: u8,
}

impl Default for Repeat {
    fn default() -> Self {
        Repeat { number: 1 }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Modifier {
    // TODO: put the operand into the opcode. Otherwise, we have to
    // supply a value when it's not needed.
    op: Opcode,
    operand: u8,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier { op: Opcode::None, operand: 0 }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Opcode {
    None,
    Plus,
    Minus,
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
        repeat: repeat.unwrap_or_default(),
        sides,
        modifier: modifier.unwrap_or_default(),
        directives: directives.unwrap_or_default(),
    };
    Ok((factor, rest))
}

/*
   repeat --> number
          -->
*/
fn parse_repeat(s: &str) -> Result<(Option<Repeat>, &str)> {
    if s.starts_with(|ch: char| ch.is_ascii_digit()) {
        // Find the first ch that is not a digit.
        // TODO: you could do this as a single call to find and skip the starts_with
        if let Some(end) = s.find(|ch: char| !ch.is_ascii_digit()) {
            let number = s[..end].parse::<u8>()?;
            let repeat = Repeat { number };
            Ok((Some(repeat), &s[end..]))
        } else {
            // This is the case where we find the beginning of the
            // repeat number, but there is nothing after it.
            Err(UnexpectedEndOfString(s.to_string()))
        }
    } else {
        Ok((None, s))
    }
}

fn parse_sides(s: &str) -> Result<(u8, &str)> {
    parse_number(s)
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
fn parse_modifier(s: &str) -> Result<(Option<Modifier>, &str)> {
    if s.starts_with('+') {
        let (operand, rest) = parse_operand(&s[1..])?;
        Ok((Some(Modifier { op: Opcode::Plus, operand}), rest))
    } else if s.starts_with('-') {
        let (operand, rest) = parse_operand(&s[1..])?;
        Ok((Some(Modifier { op: Opcode::Minus, operand}), rest))
    } else {
        Ok((None, s))
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
fn parse_directives(s: &str) -> Result<(Option<Directives>, &str)> {
    if s.starts_with('E') {
        let directives = Directives { explode: true };
        Ok((Some(directives), &s[1..]))
    } else {
        Ok((None, s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foobar() {
        let foo = parse_diecode("d6").unwrap();
        assert_eq!(foo, DieCode { factors: vec![]});
    }
}