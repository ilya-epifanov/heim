use std::fs;
use std::io;
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
