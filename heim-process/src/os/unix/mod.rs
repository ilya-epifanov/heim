//! Unix-specific extensions.

use crate::ProcessResult;

mod signal;

pub use self::signal::Signal;

/// Unix-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
pub trait ProcessExt {
    /// Send the signal to process.
    fn signal(&self, signal: Signal) -> ProcessResult<()>;

    /// Get process niceness.
    fn niceness(&self) -> ProcessResult<libc::c_int>;

    /// Set process niceness.
    fn set_niceness(&self, value: libc::c_int) -> ProcessResult<()>;
}

#[cfg(unix)]
impl ProcessExt for crate::Process {
    fn signal(&self, signal: Signal) -> ProcessResult<()> {
        self.as_ref().signal(signal)
    }

    fn niceness(&self) -> ProcessResult<libc::c_int> {
        self.as_ref().niceness()
    }

    fn set_niceness(&self, value: libc::c_int) -> ProcessResult<()> {
        self.as_ref().set_niceness(value)
    }
}
