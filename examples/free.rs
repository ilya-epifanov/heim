////! Naive clone of the `free` utility
//
//use heim::{memory, units::information, Result};
//
//fn main() -> Result<()> {
//    let memory = memory::memory()?;
//    let swap = memory::swap()?;
//
//    println!("              total        free   available");
//    println!(
//        "{:>7} {:>11?} {:>11?} {:>11?}",
//        "Mem:",
//        memory.total().get::<information::megabyte>(),
//        memory.free().get::<information::megabyte>(),
//        memory.available().get::<information::megabyte>(),
//    );
//    println!(
//        "{:>7} {:>11?} {:>11?} {:>11?}",
//        "Swap:",
//        swap.total().get::<information::megabyte>(),
//        swap.used().get::<information::megabyte>(),
//        swap.free().get::<information::megabyte>(),
//    );
//
//    Ok(())
//}
fn main() {
    unimplemented!()
}
