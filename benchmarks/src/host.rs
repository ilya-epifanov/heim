use criterion::{criterion_group, Criterion};

pub fn inner(c: &mut Criterion) {
    c.bench_function("host_platform", |b| b.iter(heim::host::platform));

    c.bench_function("host_uptime", |b| b.iter(heim::host::uptime));

    c.bench_function("host_boot_time", |b| b.iter(heim::host::boot_time));

    c.bench_function("host_users", |b| {
        b.iter(|| heim::host::users().unwrap().for_each(|_| ()))
    });
}

criterion_group!(bench, inner);
