use std::process::Command;

use crate::Result;

pub fn nproc() -> Result<u64> {
    let free = Command::new("nproc")
        .arg("--all")
        .env("LANG", "C.UTF-8")
        .output()?;
    let stdout = String::from_utf8(free.stdout)?;

    Ok(stdout.trim().parse()?)
}
