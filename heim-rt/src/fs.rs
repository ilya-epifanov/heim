use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use crate::spawn_blocking;

pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || fs::read_to_string(path)).await
}

pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr + Send + 'static,
    E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
    spawn_blocking(|| {
        let contents = fs::read_to_string(path)?;

        R::from_str(&contents).map_err(Into::into)
    })
    .await
}
