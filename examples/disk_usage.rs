////! Unix `du -h` command implementation.
//
//use std::ffi::OsStr;
//
//use heim::units::information;
//use tokio::stream::StreamExt;
//
//fn main() -> heim::Result<()> {
//    println!(
//        "{:<17} {:<10} {:<10} {:<10} {:<10} Mount",
//        "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type"
//    );
//
//    let partitions = heim::disk::partitions_physical()?;
//
//    for part in partitions {
//        let part = part?;
//        let usage = heim::disk::usage(part.mount_point().to_path_buf())?;
//
//        println!(
//            "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
//            part.device()
//                .unwrap_or_else(|| OsStr::new("N/A"))
//                .to_string_lossy(),
//            usage.total().get::<information::megabyte>(),
//            usage.used().get::<information::megabyte>(),
//            usage.free().get::<information::megabyte>(),
//            part.file_system().as_str(),
//            part.mount_point().to_string_lossy(),
//        );
//    }
//
//    Ok(())
//}

fn main() {
    unimplemented!()
}
