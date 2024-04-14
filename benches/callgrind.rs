#![allow(missing_docs)]

use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use trap_rust::compute_rain_collected;

fn setup() -> Vec<i64> {
    const N: i64 = 10000000;
    (0..N).map(|_| rand::random::<i64>() % N).collect()
}

#[library_benchmark]
#[bench::long(&setup())]
fn bench_compute_rain_collected_trap(terrain: &[i64]) -> u64 {
    std::hint::black_box(compute_rain_collected(&terrain))
}

library_benchmark_group!(name = benches;
    benchmarks= bench_compute_rain_collected_trap);

main!(library_benchmark_groups = benches);
