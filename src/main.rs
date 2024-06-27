//! # Rain trapped in terrain
//! Given a terrain represented by a sequence of integers, each
//! representing the elevation of a spot, calculate the amount of
//! rain that can be trapped in the terrain.

use trap_cpp::trap_cpp_dp;
use trap_rust::compute_rain_collected;
#[cfg(feature = "swift")]
use trap_swift::trap_swift;
use trap_zig::trap_zig_ffi;

const N: i64 = 100000;

fn main() {
    use std::time::Instant;

    let terrain: Vec<i64> = (0..N).map(|_| rand::random::<i64>() % N).collect();

    let start_time = Instant::now();
    let t1 = compute_rain_collected(&terrain);
    let execution_time = start_time.elapsed().as_micros();

    println!("Execution time trap: {} microseconds", execution_time);
    println!("Water capacity: {}", t1);

    #[cfg(feature = "swift")]
    {
        let start_time = Instant::now();
        let t1 = trap_swift(&terrain);
        let execution_time = start_time.elapsed().as_micros();

        println!("Execution time trap_swift: {} microseconds", execution_time);
        println!("Water capacity: {}", t1);
    }

    let start_time = Instant::now();
    let t1 = trap_cpp_dp(&terrain);
    let execution_time = start_time.elapsed().as_micros();

    println!(
        "Execution time trap_cpp_dp: {} microseconds",
        execution_time
    );
    println!("Water capacity: {}", t1);

    let start_time = Instant::now();
    let t1 = trap_zig_ffi(&terrain);
    let execution_time = start_time.elapsed().as_micros();

    println!(
        "Execution time trap_zig_ffi: {} microseconds",
        execution_time
    );
    println!("Water capacity: {}", t1);
}
