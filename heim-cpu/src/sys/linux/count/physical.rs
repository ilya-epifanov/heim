use std::fs;
use std::collections::HashSet;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::str;

use heim_common::prelude::*;

fn topology() -> Result<u64> {
    let mut acc = HashSet::<u64>::new();
    let mut entries = glob::glob("/sys/devices/system/cpu/cpu*/topology/core_id");
    while let Some(entry) = entries.next() {
        let entry = entry?;
        let name = entry.path().file_name();

        // TODO: Make it safe
        let core_id = unsafe { str::from_utf8_unchecked(&name.as_bytes()[3..]) };
        match core_id.parse::<u64>() {
            Ok(..) => {}
            _ => continue,
        }

        let path = entry.path().join("topology/core_id");
        let contents = rt::fs::read_to_string(path).await?;
        let cpu_id = contents.trim().parse()?;

        let _ = acc.insert(cpu_id);
    }

    if !acc.is_empty() {
        Ok(acc.len() as u64)
    } else {
        // This error will not be propagated to caller,
        // since `physical_count` will call `or_else()` on it
        Err(Error::from(io::Error::from(io::ErrorKind::InvalidData)))
    }
}

#[derive(Default)]
struct Collector {
    physical_id: Option<u64>,
    group: HashSet<(u64, u64)>,
}

fn parse_line(line: &str) -> Result<u64> {
    line.split(':')
        .nth(2)
        .map(|value| value.trim())
        .ok_or_else(|| Error::from(io::Error::from(io::ErrorKind::InvalidData)))
        .and_then(|value| value.parse::<u64>().map_err(Error::from))
}

fn cpu_info() -> Result<Option<u64>> {
    let mut acc = Collector::default();

    let mut lines = fs::read_lines("/proc/cpuinfo")?;
    while let Some(line) = lines.next() {
        match &line? {
            l if l.starts_with("physical id") => {
                let core_id = parse_line(l.as_str())?;
                if acc.physical_id.is_none() {
                    acc.physical_id = Some(core_id)
                } else {
                    // TODO: In general it seems better to return an error
                    panic!("Missed the core id value in the /proc/cpuinfo, implementation bug");
                }
            }
            l if l.starts_with("core id") => {
                let core_id = parse_line(l.as_str())?;
                if acc.physical_id.is_some() {
                    let physical_id = acc.physical_id.take().expect("Not expected to be happen");
                    let _ = acc.group.insert((physical_id, core_id));
                } else {
                    // TODO: In general it seems better to return an error
                    panic!("Missed the physical id value in the /proc/cpuinfo!");
                }
            }
            _ => continue,
        }
    }

    if !acc.group.is_empty() {
        Ok(Some(acc.group.len() as u64))
    } else {
        Ok(None)
    }
}

pub fn physical_count() -> Result<Option<u64>> {
    match topology() {
        Ok(count) => Ok(Some(count)),
        Err(..) => cpu_info(),
    }
}
