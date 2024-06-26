#![allow(missing_docs)]
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use trap_rust::{
    compute_rain_collected, compute_rain_collected3, compute_rain_collected_v,
    trap, trap_unsafe, trap_v,
};

use trap_cpp::{trap_cpp, trap_cpp_dp};
use trap_zig::trap_zig_ffi;

fn bench_compute_rain_collected_trap(c: &mut Criterion) {
    const N: i64 = 10000000;
    let terrain: Vec<i64> = (0..N).map(|_| rand::random::<i64>() % N).collect();
    let mut group = c.benchmark_group("compute_rain_collected_trap");
    group.measurement_time(Duration::from_secs(6));
    group.bench_function("compute_rain_collected", |b| {
        b.iter(|| compute_rain_collected(&terrain))
    });
    group.bench_function("trap", |b| b.iter(|| trap(&terrain)));
    group.bench_function("trap_unsafe", |b| b.iter(|| trap_unsafe(&terrain)));
    group.bench_function("trap_cpp", |b| b.iter(|| trap_cpp(&terrain)));
    group.bench_function("trap_cpp_dp", |b| b.iter(|| trap_cpp_dp(&terrain)));
    #[cfg(feature = "swift")]
    {
        use trap_swift::trap_swift;
        group.bench_function("trap_swift", |b| b.iter(|| trap_swift(&terrain)));
    }
    group.bench_function("trap_zig", |b| b.iter(|| trap_zig_ffi(&terrain)));

    group.bench_function("compute_rain_collected_v", |b| {
        b.iter_batched(
            || terrain.clone(),
            |t| compute_rain_collected_v(t),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("trap_v", |b| {
        b.iter_batched(
            || terrain.clone(),
            |t| trap_v(t),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("compute_rain_collected3", |b| {
        b.iter(|| compute_rain_collected3(&terrain))
    });

    group.finish();
}

criterion_group!(benches, bench_compute_rain_collected_trap);
criterion_main!(benches);
