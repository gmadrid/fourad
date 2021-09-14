use argh::FromArgs;
use std::io::{BufRead, BufReader};

// TODO: improve output formatting.
// TODO: change 'explode' and 'explodes' to be consistent.

#[derive(FromArgs)]
/// A dice roller for _Four Against Darkness_
struct Args {
    #[argh(positional)]
    codes: Vec<String>,

    /// if set, all d6 rolls will 'explode' (keep rolling on a '6').
    #[argh(switch, short = 'X')]
    explode: bool,
}

fn process_stdin(explodes: bool) -> fourad::Result<()> {
    let input = BufReader::new(std::io::stdin());

    for line in input.lines() {
        let line = line?;
        output_code(&line, explodes, true)?;
    }
    Ok(())
}

fn output_code(s: &str, explodes: bool, print_codes: bool) -> fourad::Result<()> {
    if print_codes {
        println!("{}", s);
    }
    println!("===> {}", fourad::roll(&s, explodes)?);
    if print_codes {
        println!()
    }
    Ok(())
}

// TODO: BUG! --explodes shouldn't make d8 re-roll on 6.

fn main() -> fourad::Result<()> {
    let args: Args = argh::from_env();

    if args.codes.is_empty() {
        return process_stdin(args.explode);
    }

    let print_codes = args.codes.len() > 1;

    for code in args.codes {
        output_code(&code, args.explode, print_codes)?;
    }
    Ok(())
}
