#![allow(missing_docs)]
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use rain_collected::{
    compute_rain_collected, compute_rain_collected2, compute_rain_collected3, trap,
};

fn bench_compute_rain_collected_trap(c: &mut Criterion) {
    const N: i64 = 100000;
    let terrain: Vec<i64> = (0..N).map(|_| (rand::random::<i64>() % N)).collect();
    let mut group = c.benchmark_group("compute_rain_collected_trap");
    group.measurement_time(Duration::from_secs(6));
    group.bench_function("compute_rain_collected", |b| {
        b.iter(|| compute_rain_collected(&terrain))
    });
    group.bench_function("trap", |b| b.iter(|| trap(&terrain)));
    group.bench_function("compute_rain_collected2", |b| {
        b.iter_batched(
            || terrain.clone(),
            |t| compute_rain_collected2(t),
            criterion::BatchSize::LargeInput,
        )
    });
    group.bench_function("compute_rain_collected3", |b| {
        b.iter(|| compute_rain_collected3(&terrain))
    });

    group.finish();
}

criterion_group!(benches, bench_compute_rain_collected_trap);
criterion_main!(benches);