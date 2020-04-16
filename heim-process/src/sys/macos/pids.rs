use super::bindings;
use crate::sys::unix;
use crate::{Pid, ProcessResult};

pub fn pids() -> ProcessResult<impl Iterator<Item = ProcessResult<Pid>>> {
    let processes = bindings::processes()?;
    let iter = processes.into_iter().map(|proc| Ok(proc.kp_proc.p_pid));

    Ok(iter)
}

pub fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    Ok(unix::pid_exists(pid))
}
