use argh::FromArgs;
use fourad::{quiet, spew, SpewLevel};
use std::io::{BufRead, BufReader};

// TODO: improve output formatting.

#[derive(FromArgs)]
/// A dice roller for _Four Against Darkness_
struct Args {
    #[argh(positional)]
    codes: Vec<String>,

    /// if set, all d6 rolls will 'explode' (keep rolling on a '6').
    #[argh(switch, short = 'X')]
    explode: bool,

    /// if set, d66 will be treated as a 66-sided die
    #[argh(switch)]
    force_66: bool,

    /// if set, run with minimal output
    #[argh(switch, short = 'q')]
    quiet: bool,

    /// if set, run with lots of output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

fn process_stdin(explode: bool, force_66: bool) -> fourad::Result<()> {
    BufReader::new(std::io::stdin())
        .lines()
        .map(|line_wrapped| {
            let line = line_wrapped?;
            output_code(&line, explode, true, force_66)
        })
        .collect()
}

fn output_code(s: &str, explode: bool, print_codes: bool, force_66: bool) -> fourad::Result<()> {
    if print_codes {
        spew!("{}", s);
    }
    // TODO: this prints too much when "quiet".
    quiet!("===> {}", fourad::roll(s, explode, force_66)?);
    if print_codes {
        spew!("")
    }
    Ok(())
}

fn set_spew_level(args: &Args) -> fourad::Result<()> {
    if args.quiet && args.verbose {
        return Err(fourad::Error::GeneralError(
            "--quiet and --vesbose are not compatible.".to_string(),
        ));
    }

    fourad::set_level(if args.quiet {
        SpewLevel::QUIET
    } else if args.verbose {
        SpewLevel::VERBOSE
    } else {
        SpewLevel::STANDARD
    });
    Ok(())
}

fn main() -> fourad::Result<()> {
    let args: Args = argh::from_env();

    set_spew_level(&args)?;

    if args.codes.is_empty() {
        return process_stdin(args.explode, args.force_66);
    }

    let print_codes = args.codes.len() > 1;

    for code in args.codes {
        output_code(&code, args.explode, print_codes, args.force_66)?;
    }
    Ok(())
}
