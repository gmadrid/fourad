use std::str::FromStr;

mod parser;
use parser::parse_diecode;

use crate::{Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct RollDesc {
    pub repeat: u8,
    pub sides: u8,
    pub modifier: RollModifier,
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

#[derive(Debug, Eq, PartialEq)]
pub enum RollModifier {
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

impl FromStr for RollDesc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_diecode(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_mod() {
        assert_eq!(RollModifier::None, RollModifier::default());
    }

    #[test]
    fn default_desc() {
        assert_eq!(
            RollDesc::default(),
            RollDesc {
                repeat: 1,
                sides: 6,
                modifier: Default::default()
            }
        )
    }

    #[test]
    fn basic() {
        // We just ensure that parsing works. All of the separates cases are tested in parser.rs.
        // No fields are defaults.
        assert_eq!(
            "2d8+3".parse::<RollDesc>().unwrap(),
            RollDesc {
                repeat: 2,
                sides: 8,
                modifier: RollModifier::Plus(3)
            }
        )
    }
}
