use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns [Time] amount from the system boot.
///
/// [Time]: ./struct.Time.html
pub fn uptime() -> Result<Time> {
    sys::uptime()
}
