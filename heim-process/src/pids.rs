use crate::{sys, Pid, ProcessResult};

/// Returns a stream over the [Pid]s of the processes currently running in the system.
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// [Pid]: type.Pid.html
pub fn pids() -> ProcessResult<impl Iterator<Item = ProcessResult<Pid>>> {
    let inner = sys::pids()?;

    Ok(inner)
}

/// Checks if the process with given `pid` exists.
pub fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    sys::pid_exists(pid)
}
