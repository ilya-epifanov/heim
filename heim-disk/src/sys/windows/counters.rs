use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::{information, time, Information, Time};

use super::bindings;

#[derive(Debug)]
pub struct IoCounters {
    volume_path: PathBuf,
    read_count: u64,
    write_count: u64,
    read_bytes: Information,
    write_bytes: Information,
    read_time: Time,
    write_time: Time,
}

impl IoCounters {
    pub fn device_name(&self) -> &OsStr {
        self.volume_path.as_os_str()
    }

    pub fn read_count(&self) -> u64 {
        self.read_count
    }

    pub fn write_count(&self) -> u64 {
        self.write_count
    }

    pub fn read_bytes(&self) -> Information {
        self.read_bytes
    }

    pub fn write_bytes(&self) -> Information {
        self.write_bytes
    }

    pub fn read_time(&self) -> Time {
        self.read_time
    }

    pub fn write_time(&self) -> Time {
        self.write_time
    }
}

fn inner_iter<F>(mut filter: F) -> Result<impl Iterator<Item = Result<IoCounters>>>
where
    F: FnMut(&Path) -> bool + 'static,
{
    let volumes = bindings::Volumes::new().filter_map(move |try_path| {
        match try_path {
            Ok(volume_path) => {
                if !filter(&volume_path) {
                    return None;
                }

                let perf = match bindings::disk_performance(&volume_path) {
                    Ok(Some(perf)) => perf,
                    Ok(None) => return None,
                    Err(e) => return Some(Err(e)),
                };

                let read_bytes = unsafe { *perf.BytesRead.QuadPart() as u64 };
                let write_bytes = unsafe { *perf.BytesWritten.QuadPart() as u64 };
                let read_time = unsafe { *perf.ReadTime.QuadPart() as f64 };
                let write_time = unsafe { *perf.WriteTime.QuadPart() as f64 };

                let counters = IoCounters {
                    volume_path,
                    read_count: perf.ReadCount.into(),
                    write_count: perf.WriteCount.into(),
                    read_bytes: Information::new::<information::byte>(read_bytes),
                    write_bytes: Information::new::<information::byte>(write_bytes),
                    // `ReadTime` and `WriteTime` seems to be in tenths of microseconds
                    // https://github.com/giampaolo/psutil/issues/1012
                    read_time: Time::new::<time::microsecond>(read_time * 10.0),
                    write_time: Time::new::<time::microsecond>(write_time * 10.0),
                };

                Some(Ok(counters))
            }
            Err(e) => Some(Err(e)),
        }
    });

    Ok(volumes)
}

pub fn io_counters() -> Result<impl Iterator<Item = Result<IoCounters>>> {
    Ok(inner_iter(|_| true)?)
}

pub fn io_counters_physical() -> Result<impl Iterator<Item = Result<IoCounters>>> {
    Ok(inner_iter(|path: &Path| {
        bindings::DriveType::from_path(path) == Some(bindings::DriveType::Fixed)
    })?)
}
