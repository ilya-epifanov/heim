use std::fs;

use heim_common::{
    units::{time, Time},
    Error, Result,
};

const PROC_STAT: &str = "/proc/stat";

pub fn boot_time() -> Result<Time> {
    let contents = fs::read_to_string(PROC_STAT)?;

    for line in contents.lines() {
        if line.starts_with("btime ") {
            let mut parts = line.splitn(2, ' ');
            let _ = parts.next();

            return match parts.next() {
                Some(raw_value) => raw_value
                    .parse::<f64>()
                    .map(Time::new::<time::second>)
                    .map_err(Into::into),
                None => Err(Error::missing_key("btime", PROC_STAT)),
            };
        }
    }

    Err(Error::missing_key("btime", PROC_STAT))
}
