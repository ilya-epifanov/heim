use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

/// Read file contents and parse them into `T`.
pub fn read_into<P, T, E>(path: P) -> Result<T, E>
where
    P: AsRef<Path>,
    T: FromStr<Err = E>,
    E: From<io::Error>,
{
    let contents = fs::read_to_string(path)?;

    T::from_str(&contents)
}

pub fn read_lines<P>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let reader = io::BufReader::new(file);

    Ok(reader.lines())
}

pub fn read_lines_into<P, T, E>(path: P) -> io::Result<impl Iterator<Item = Result<T, E>>>
where
    P: AsRef<Path>,
    T: FromStr<Err = E>,
    E: From<io::Error>,
{
    let lines = read_lines(path)?;
    let iter = lines.map(|res| match res {
        Ok(value) => T::from_str(&value),
        Err(e) => Err(e.into()),
    });

    Ok(iter)
}
