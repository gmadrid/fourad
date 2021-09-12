fn main() -> fourad::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "d6".to_string());

    println!("{}", fourad::roll(&arg)?);
    Ok(())
}
