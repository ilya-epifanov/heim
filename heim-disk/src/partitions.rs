use std::ffi::OsStr;
use std::fmt;
use std::path::Path;

use heim_common::prelude::*;

use crate::{sys, FileSystem};

/// Mounted disk partition.
///
/// ## Compatibility
///
/// See [os]-specific extension traits also.
///
/// [os]: ./os/index.html
pub struct Partition(sys::Partition);

wrap!(Partition, sys::Partition);

impl Partition {
    /// Returns partition device name if available.
    pub fn device(&self) -> Option<&OsStr> {
        self.as_ref().device()
    }

    /// Returns partition mount point path.
    pub fn mount_point(&self) -> &Path {
        self.as_ref().mount_point()
    }

    /// Returns partition file system.
    pub fn file_system(&self) -> &FileSystem {
        self.as_ref().file_system()
    }
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Partition")
            .field("device", &self.device())
            .field("mount_point", &self.mount_point())
            .field("file_system", &self.file_system())
            .finish()
    }
}

/// Returns a stream over mounted disk [Partitions].
///
/// This includes all virtual partitions, such as `tmpfs`.
/// See [partitions_physical] for physical partitions stream.
///
/// [Partitions]: struct.Partition.html
pub fn partitions() -> Result<impl Iterator<Item = Result<Partition>>> {
    let inner = sys::partitions()?;
    Ok(inner.map(|r| r.map(Into::into)))
}

/// Returns a stream over physical only mounted disk [Partitions].
///
/// [Partitions]: struct.Partition.html
pub fn partitions_physical() -> Result<impl Iterator<Item = Result<Partition>>> {
    let inner = sys::partitions_physical()?;
    Ok(inner.map(|r| r.map(Into::into)))
}
