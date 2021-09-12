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

fn main() -> fourad::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "d6".to_string());

    println!("{}", fourad::roll(&arg)?);
    Ok(())
}
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn basic_d6() {
//         assert_eq!("d6".parse::<RollDesc>().unwrap(), RollDesc::default());
//     }
//
//     #[test]
//     fn repeat_2d6() {
//         assert_eq!(
//             "2d6".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 repeat: 2,
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn modifier_d6plus1() {
//         assert_eq!(
//             "d6+1".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 modifier: RollModifier::Plus(1),
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn repeat_modifier_2d6plus1() {
//         assert_eq!(
//             "2d6+1".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 repeat: 2,
//                 modifier: RollModifier::Plus(1),
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn test_2d6plus2() {
//         assert_eq!(
//             "2d6+2".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 repeat: 2,
//                 modifier: RollModifier::Plus(2),
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn test_d6xd6() {
//         assert_eq!(
//             "d6xd6".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 modifier: RollModifier::Squared,
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn test_d6minus1() {
//         assert_eq!(
//             "d6-1".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 modifier: RollModifier::Minus(1),
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn test_d6minus2() {
//         assert_eq!(
//             "d6-2".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 modifier: RollModifier::Minus(2),
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn hundo_d66() {
//         assert_eq!(
//             "d66".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 modifier: RollModifier::Hundo,
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     #[test]
//     fn test_d3() {
//         assert_eq!(
//             "d3".parse::<RollDesc>().unwrap(),
//             RollDesc {
//                 sides: 3,
//                 ..RollDesc::default()
//             }
//         )
//     }
//
//     // TODO: test execute()
// }
