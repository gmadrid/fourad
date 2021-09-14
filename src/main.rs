use argh::FromArgs;

// TODO: improve output formatting.

#[derive(FromArgs)]
/// A dice roller for _Four Against Darkness_
struct Args {
    #[argh(positional)]
    codes: Vec<String>,
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
        println!("===> {}", fourad::roll(&code, true)?);
        if print_codes {
            println!()
        }
    }
    Ok(())
}
