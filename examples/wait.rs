////! Await process completion, similar to `wait` command.
////!
////! Process pid should be passed as a program argument, ex.
////!
////! ```
////! $ cargo run --example process -- $$
////! ```
////!
////! `$$` is expanded by bash into its own pid.
//
//use std::env;
//use std::error::Error;
//use std::io;
//
//use heim::process;
//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn Error>> {
//    let pid = env::args()
//        .nth(1)
//        .ok_or_else(|| {
//            eprintln!("Process PID is not passed as a CLI argument");
//            io::Error::from(io::ErrorKind::InvalidInput)
//        })?
//        .parse::<process::Pid>()?;
//
//    let process = process::get(pid).await?;
//
//    println!("Watching for process {} completion", pid);
//    process.wait().await?;
//    println!("Process {} had exited", pid);
//
//    Ok(())
//}

fn main() {
    unimplemented!()
}
