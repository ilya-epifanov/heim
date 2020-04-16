use std::fmt;
use std::path::PathBuf;
use std::time::Instant;

use heim_common::prelude::*;
use heim_common::units::Time;

use crate::{sys, Pid, ProcessResult};

mod command;
mod cpu_times;
mod cpu_usage;
mod env;
mod memory;
mod status;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
pub use self::cpu_usage::CpuUsage;
pub use self::env::{Environment, EnvironmentIter, IntoEnvironmentIter};
pub use self::memory::Memory;
pub use self::status::Status;

/// System process.
///
/// Some extra methods can be found in the [OS extensions](./os/index.html)
#[derive(Eq, PartialEq, Hash)]
pub struct Process(sys::Process);

wrap!(Process, sys::Process);

impl Process {
    /// Returns the process pid.
    pub fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    /// Returns process parent pid.
    pub fn parent_pid(&self) -> ProcessResult<Pid> {
        self.as_ref().parent_pid()
    }

    /// Returns parent [Process].
    ///
    /// [Process]: ./struct.Process.html
    pub fn parent(&self) -> ProcessResult<Process> {
        let ppid = self.parent_pid()?;

        get(ppid)
    }

    /// Returns process name.
    pub fn name(&self) -> ProcessResult<String> {
        self.as_ref().name()
    }

    /// Returns process executable as an absolute path.
    pub fn exe(&self) -> ProcessResult<PathBuf> {
        self.as_ref().exe()
    }

    /// Returns process command line.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use heim_process::{self as process, Process, ProcessResult};
    /// #
    /// # #[heim_derive::main]
    /// # fn main() -> ProcessResult<()> {
    /// let process = process::current()?;
    /// let command = process.command()?;
    /// println!("Command line arguments:");
    /// for arg in &command {
    ///     println!("{:?}", arg);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn command(&self) -> ProcessResult<Command> {
        self.as_ref().command().map(Into::into)
    }

    /// Returns process current working directory.
    ///
    /// ## Compatibility
    ///
    /// For Windows this method is not implemented yet and will always panic,
    /// see [#105](https://github.com/heim-rs/heim/issues/105).
    pub fn cwd(&self) -> ProcessResult<PathBuf> {
        self.as_ref().cwd()
    }

    /// Returns current process status.
    pub fn status(&self) -> ProcessResult<Status> {
        self.as_ref().status()
    }

    /// Returns process environment.
    pub fn environment(&self) -> ProcessResult<Environment> {
        self.as_ref().environment().map(Into::into)
    }

    /// Returns process creation time, expressed as a [Time] amount since the UNIX epoch.
    ///
    /// [Time]: ../units/type.Time.html
    pub fn create_time(&self) -> ProcessResult<Time> {
        self.as_ref().create_time()
    }

    /// Returns accumulated process time.
    pub fn cpu_time(&self) -> ProcessResult<CpuTime> {
        self.as_ref().cpu_time().map(Into::into)
    }

    /// Returns CPU usage measurement.
    ///
    /// Returned [`CpuUsage`] struct represents instantaneous CPU usage and does not represent
    /// any reasonable value by itself.
    /// It is suggested to wait for a while with help of any timer
    /// (for accuracy recommended delay should be at least 100 ms),
    /// call this method once again and subtract former [`CpuUsage`] from the new one.
    ///
    /// Same to any *nix system, calculated CPU usage might exceed 100 %
    /// if the process is running multiple threads on different CPU cores.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use heim_common::units::ratio;
    /// # use heim_process::{self as process, Process, ProcessResult};
    /// #
    /// # #[heim_derive::main]
    /// # fn main() -> ProcessResult<()> {
    /// let process = process::current()?;
    /// let measurement_1 = process.cpu_usage()?;
    /// // Or any other timer at your choice
    /// futures_timer::Delay::new(Duration::from_millis(100));
    /// let measurement_2 = process.cpu_usage()?;
    ///
    /// println!("CPU usage: {} %", (measurement_2 - measurement_1).get::<ratio::percent>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`CpuUsage`]: ./struct.CpuUsage.html
    pub fn cpu_usage(&self) -> ProcessResult<CpuUsage> {
        let cpu_time = self.cpu_time()?;
        let cpu_count = heim_cpu::logical_count()?;

        Ok(CpuUsage {
            cpu_count,
            cpu_time,
            at: Instant::now(),
        })
    }

    /// Returns memory usage information for this process.
    pub fn memory(&self) -> ProcessResult<Memory> {
        self.as_ref().memory().map(Into::into)
    }

    /// Checks if this `Process` is still running.
    pub fn is_running(&self) -> ProcessResult<bool> {
        self.as_ref().is_running()
    }

    /// Suspends the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGSTOP` signal to process.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    pub fn suspend(&self) -> ProcessResult<()> {
        self.as_ref().suspend()
    }

    /// Resumes the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGCONT` signal to process.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    pub fn resume(&self) -> ProcessResult<()> {
        self.as_ref().resume()
    }

    /// Terminates the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGTERM` signal to process.
    ///
    /// For Windows it is an alias for the [`Process::kill`]
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    /// [`Process::kill`]: #method.kill
    pub fn terminate(&self) -> ProcessResult<()> {
        self.as_ref().terminate()
    }

    /// Kills the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGKILL` signal to process.
    ///
    /// [`TerminateProcess`] function is used for Windows,
    /// it initiates the termination but does not awaits for completion.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    /// [`TerminateProcess`]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess
    pub fn kill(&self) -> ProcessResult<()> {
        self.as_ref().kill()
    }

    /// Wait for the current process termination.
    ///
    /// ## Returns
    ///
    /// If the process is already terminated, this method returns `Ok(())`.
    pub fn wait(&self) -> ProcessResult<()> {
        self.as_ref().wait()
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid()).finish()
    }
}

/// Returns an iterator over the currently running processes.
pub fn processes() -> ProcessResult<impl Iterator<Item = ProcessResult<Process>>> {
    let inner = sys::processes()?;

    Ok(inner.map(|r| r.map(Process::from)))
}

/// Loads the process information with `pid` given.
pub fn get(pid: Pid) -> ProcessResult<Process> {
    sys::get(pid).map(Into::into)
}

/// Returns the `Process` matching the currently running program.
pub fn current() -> ProcessResult<Process> {
    sys::current().map(Into::into)
}
