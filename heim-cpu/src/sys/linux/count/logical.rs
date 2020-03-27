use heim_common::prelude::*;

fn sysconf() -> Result<u64> {
    let result = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };

    if result < 0 {
        Err(Error::last_os_error().with_sysconf(libc::_SC_NPROCESSORS_ONLN))
    } else {
        Ok(result as u64)
    }
}

fn cpuinfo() -> Result<u64> {
    let mut count = 0;
    for line in fs_ext::read_lines("/proc/cpuinfo")? {
        if line?.starts_with("processor") {
            count += 1;
        }
    }

    Ok(count)
}

fn stat() -> Result<u64> {
    // the first "cpu" line aggregates the numbers in all
    // of the other "cpuN" lines, hence skip the first item
    let mut count = 0;
    for line in fs_ext::read_lines("/proc/stat")?.skip(1) {
        if line?.starts_with("cpu") {
            count += 1;
        }
    }

    Ok(count)
}

pub fn logical_count() -> Result<u64> {
    match sysconf() {
        Ok(value) => Ok(value),
        Err(..) => match cpuinfo() {
            Ok(value) => Ok(value),
            Err(..) => stat(),
        },
    }
}
