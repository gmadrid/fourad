use argh::FromArgs;

// TODO: improve output formatting.

#[derive(FromArgs)]
/// A dice roller for _Four Against Darkness_
struct Args {
    #[argh(positional)]
    codes: Vec<String>,

    /// if set, all d6 rolls will 'explode' (keep rolling on a '6').
    #[argh(switch, short = 'X')]
    explode: bool,
}

fn main() -> fourad::Result<()> {
    let args: Args = argh::from_env();

    // TODO: read from stdin if len == 0.
    let print_codes = args.codes.len() > 1;

    for code in args.codes {
        if print_codes {
            println!("{}", code);
        }
        // TODO: add an --explodes switch
        println!("===> {}", fourad::roll(&code, args.explode)?);
        if print_codes {
            println!()
        }
    }
    Ok(())
}
