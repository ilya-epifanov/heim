use std::fs;

use heim_common::{
    units::{time, Time},
    Error, Result,
};

const PROC_UPTIME: &str = "/proc/uptime";

pub fn uptime() -> Result<Time> {
    let contents = fs::read_to_string(PROC_UPTIME)?;

    match contents.splitn(2, ' ').next() {
        Some(raw_value) => {
            let seconds = raw_value.parse::<f64>()?;

            Ok(Time::new::<time::second>(seconds))
        }
        None => Err(Error::missing_key("uptime", "/proc/uptime")),
    }
}
