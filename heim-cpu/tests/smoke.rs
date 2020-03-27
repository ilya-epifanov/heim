////#![feature(test)]
//
//use heim_common::prelude::*;
//use heim_common::units::frequency;
use heim_cpu as cpu;
//use heim_runtime as rt;
//
//#[heim_derive::skip_ci(target_os = "linux")]
//fn smoke_frequency() {
//    let freq = cpu::frequency().unwrap();
//
//    assert!(freq.current().get::<frequency::hertz>() > 0);
//    let _ = freq.min();
//    let _ = freq.max();
//}
//
////#[cfg(target_os = "linux")]
////fn smoke_frequencies() {
////    let frequencies = cpu::os::linux::frequencies();
////    while let Some(freq) = frequencies.next().await {
////        let f = freq.unwrap();
////
////        let _ = f.current();
////        let _ = f.min();
////        let _ = f.max();
////    }
////}
//
//fn smoke_stats() {
//    let stats = cpu::stats().unwrap();
//
//    let _ = stats.ctx_switches();
//    let _ = stats.interrupts();
//
//    #[cfg(target_os = "linux")]
//    {
//        use heim_cpu::os::linux::CpuStatsExt;
//
//        let _ = stats.soft_interrupts();
//    }
//
//    #[cfg(target_os = "macos")]
//    {
//        use heim_cpu::os::macos::CpuStatsExt;
//
//        let _ = stats.soft_interrupts();
//        let _ = stats.syscalls();
//    }
//
//    #[cfg(target_os = "windows")]
//    {
//        use heim_cpu::os::windows::CpuStatsExt;
//
//        let _ = stats.dpc();
//        let _ = stats.syscalls();
//    }
//}
//
//#[heim_derive::test]
//async fn smoke_time() {
//    let time = cpu::time().await;
//    let time = time.unwrap();
//
//    let _ = time.system();
//    let _ = time.user();
//    let _ = time.idle();
//
//    #[cfg(target_os = "linux")]
//    {
//        use heim_cpu::os::linux::CpuTimeExt;
//
//        let _ = time.nice();
//        let _ = time.io_wait();
//        let _ = time.irq();
//        let _ = time.soft_irq();
//        let _ = time.steal();
//        let _ = time.guest();
//        let _ = time.guest_nice();
//    }
//}
//
//#[heim_derive::test]
//async fn smoke_times() {
//    let times = cpu::times();
//    rt::pin!(times);
//    while let Some(time) = times.next().await {
//        let time = time.unwrap();
//
//        let _ = time.system();
//        let _ = time.user();
//        let _ = time.idle();
//
//        #[cfg(target_os = "linux")]
//        {
//            use heim_cpu::os::linux::CpuTimeExt;
//
//            let _ = time.nice();
//            let _ = time.io_wait();
//            let _ = time.irq();
//            let _ = time.soft_irq();
//            let _ = time.steal();
//            let _ = time.guest();
//            let _ = time.guest_nice();
//        }
//    }
//}
//

#[test]
fn smoke_cpu_logical_count() {
    let count = cpu::logical_count().unwrap();

    assert!(count > 0);
}
//
//fn smoke_cpu_physical_count() {
//    let count = cpu::physical_count();
//    assert!(count.is_ok(), "cpu::physical_count failed: {:#?}", count);
//    let count = count.unwrap();
//
//    if let Some(cpus) = count {
//        assert!(cpus > 0);
//    }
//}
