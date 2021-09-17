use std::io::{BufRead, BufReader};

pub fn process_stdin<E>(
    f: impl FnMut(&str) -> std::result::Result<(), E>,
) -> std::result::Result<(), E>
where
    E: From<std::io::Error>,
{
    process_bufread(&mut BufReader::new(std::io::stdin()), f)
}

pub fn process_bufread<E>(
    bufread: &mut impl BufRead,
    mut f: impl FnMut(&str) -> std::result::Result<(), E>,
) -> std::result::Result<(), E>
where
    E: From<std::io::Error>,
{
    bufread.lines().try_for_each(|wrapped| f(&wrapped?))
}
