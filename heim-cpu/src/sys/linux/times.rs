use std::str::{self, FromStr};

use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;
use heim_common::units::{time, Time};

#[derive(Debug, Default)]
pub struct CpuTime {
    user: Time,
    nice: Time,
    system: Time,
    idle: Time,
    io_wait: Time,
    irq: Time,
    soft_irq: Time,
    steal: Time,
    guest: Option<Time>,
    guest_nice: Option<Time>,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.user
    }
    pub fn nice(&self) -> Time {
        self.nice
    }
    pub fn system(&self) -> Time {
        self.system
    }
    pub fn idle(&self) -> Time {
        self.idle
    }
    pub fn io_wait(&self) -> Time {
        self.io_wait
    }
    pub fn irq(&self) -> Time {
        self.irq
    }
    pub fn soft_irq(&self) -> Time {
        self.soft_irq
    }
    pub fn steal(&self) -> Time {
        self.steal
    }
    pub fn guest(&self) -> Option<Time> {
        self.guest
    }
    pub fn guest_nice(&self) -> Option<Time> {
        self.guest_nice
    }
}

impl FromStr for CpuTime {
    type Err = Error;

    // Parse one line from the /proc/stat, ex.
    // "cpu1 317865 456 71065 3101075 8645 14938 10567 0 0 0"
    fn from_str(value: &str) -> Result<CpuTime> {
        let mut times = CpuTime::default();

        let parts = value.split_whitespace().skip(1);
        for (idx, part) in parts.enumerate() {
            let value = part.parse::<u32>().map(|value| {
                let value = f64::from(value) / *CLOCK_TICKS;
                Time::new::<time::second>(value)
            })?;

            match idx {
                0 => times.user = value,
                1 => times.nice = value,
                2 => times.system = value,
                3 => times.idle = value,
                4 => times.io_wait = value,
                5 => times.irq = value,
                6 => times.soft_irq = value,
                7 => times.steal = value,
                8 => times.guest = Some(value),
                9 => times.guest_nice = Some(value),
                _ => break,
            };
        }

        Ok(times)
    }
}

pub fn time() -> Result<CpuTime> {
    // cumulative time is always the first line
    let mut lines = fs_ext::read_lines_into::<_, CpuTime, _>("/proc/stat")?;
    match lines.next() {
        Some(line) => line,
        None => Err(Error::missing_key("cumulative time line", "/proc/stat")),
    }
}

pub fn times() -> Result<impl Iterator<Item = Result<CpuTime>>> {
    let iter = fs_ext::read_lines("/proc/stat")?
        .skip(1) // Skipping cumulative time at the first line
        .filter_map(|try_line| {
            match try_line {
                Ok(l) if l.starts_with("cpu") => {
                    Some(CpuTime::from_str(&l))
                },
                Ok(..) => None,
                Err(e) => Some(Err(e.into())),
            }
        });

    Ok(iter)
}
