use std::ffi::{CStr, OsStr};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use heim_common::prelude::*;

use super::bindings;
use crate::FileSystem;

#[derive(Debug)]
pub struct Partition {
    device: String,
    fs: FileSystem,
    mount_point: PathBuf,
    flags: u32,
}

impl Partition {
    pub fn device(&self) -> Option<&OsStr> {
        Some(OsStr::new(self.device.as_str()))
    }

    pub fn mount_point(&self) -> &Path {
        self.mount_point.as_path()
    }

    pub fn file_system(&self) -> &FileSystem {
        &self.fs
    }

    pub fn raw_flags(&self) -> u32 {
        self.flags
    }
}

// TODO: Since `from` may fail in fact, replace it with a `try_from`
// See `FileSystem::from_str` in the implementation
impl From<libc::statfs> for Partition {
    fn from(stat: libc::statfs) -> Partition {
        let device = unsafe {
            CStr::from_ptr(stat.f_mntfromname.as_ptr())
                .to_string_lossy()
                .to_string()
        };
        let fs_type = unsafe { CStr::from_ptr(stat.f_fstypename.as_ptr()).to_string_lossy() };
        let mount_path_raw = unsafe {
            CStr::from_ptr(stat.f_mntonname.as_ptr())
                .to_string_lossy()
                .to_string()
        };
        let mount_point = PathBuf::from(mount_path_raw);

        let fs = FileSystem::from_str(&fs_type)
            .expect("For some stupid reasons failed to parse FS string");

        Partition {
            device,
            fs,
            mount_point,
            flags: stat.f_flags,
        }
    }
}

pub fn partitions() -> Result<impl Iterator<Item = Result<Partition>>> {
    let mounts = bindings::mounts()?;

    let iter = mounts.into_iter().map(|mount| Ok(Partition::from(mount)));

    Ok(iter)
}

pub fn partitions_physical() -> Result<impl Iterator<Item = Result<Partition>>> {
    let iter = partitions()?.filter(|try_part| match try_part {
        Ok(part) if part.file_system().is_physical() => true,
        Ok(..) => false,
        Err(..) => true,
    });

    Ok(iter)
}
