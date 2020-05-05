use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    c.bench_function("host_platform", |b| {
        b.iter(|| smol::block_on(heim::host::platform()))
    });

    c.bench_function("host_uptime", |b| {
        b.iter(|| smol::block_on(heim::host::uptime()))
    });

    c.bench_function("host_boot_time", |b| {
        b.iter(|| smol::block_on(heim::host::boot_time()))
    });

    c.bench_function("host_users", |b| {
        b.iter(|| {
            let stream = heim::host::users().for_each(|_| async {});
            smol::block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
