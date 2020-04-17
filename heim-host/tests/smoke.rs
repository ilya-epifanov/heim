use std::error::Error;

use claim::*;

use heim_common::units::time;
use heim_host as host;

#[test]
fn smoke_platform() -> Result<(), Box<dyn Error>> {
    let platform = host::platform()?;
    let _ = platform.system();
    let _ = platform.release();
    let _ = platform.version();
    let _ = platform.architecture();

    Ok(())
}

#[test]
fn smoke_uptime() -> Result<(), Box<dyn Error>> {
    let uptime = host::uptime()?;

    assert_gt!(uptime.get::<time::second>(), 0.0);

    Ok(())
}

#[test]
fn smoke_boot_time() -> Result<(), Box<dyn Error>> {
    let boot_time = host::boot_time()?;

    assert_gt!(boot_time.get::<time::second>(), 0.0);

    Ok(())
}

#[test]
#[ignore]  // TODO: Broken for Windows environments, heap corruption?
fn smoke_users() -> Result<(), Box<dyn Error>> {
    for user in host::users()? {
        let user = user?;

        let _ = user.username();

        #[cfg(target_os = "linux")]
        {
            use heim_host::os::linux::UserExt;

            let _ = user.pid();
            let _ = user.terminal();
            let _ = user.id();
            let _ = user.hostname();
            let _ = user.address();
            let _ = user.session_id();
        }

        #[cfg(target_os = "macos")]
        {
            use heim_host::os::macos::UserExt;

            let _ = user.pid();
            let _ = user.terminal();
            let _ = user.id();
            let _ = user.hostname();
        }

        #[cfg(target_os = "windows")]
        {
            use heim_host::os::windows::UserExt;

            let _ = user.domain();
            let _ = user.address();
        }
    }

    Ok(())
}
