//! Windows-specific extensions.

use crate::ProcessResult;

mod memory;
mod priority;

pub use self::memory::MemoryExt;
pub use self::priority::Priority;

/// Windows-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
pub trait ProcessExt {
    /// Get process priority.
    fn priority(&self) -> ProcessResult<Priority>;

    /// Set process priority.
    fn set_priority(&self, value: Priority) -> ProcessResult<()>;
}

#[cfg(target_os = "windows")]
impl ProcessExt for crate::Process {
    fn priority(&self) -> ProcessResult<Priority> {
        self.as_ref().priority()
    }

    fn set_priority(&self, value: Priority) -> ProcessResult<()> {
        self.as_ref().set_priority(value)
    }
}
