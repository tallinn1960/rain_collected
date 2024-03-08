//! # Rain trapped in terrain
//! Given a terrain represented by a sequence of integers, each
//! representing the elevation of a spot, calculate the amount of
//! rain that can be trapped in the terrain.

use rain_collected::{compute_rain_collected, trap_cpp_dp};
const N: i64 = 100000;

fn main() {
    use std::time::Instant;

    let terrain: Vec<i64> = (0..N).map(|_| (rand::random::<i64>() % N)).collect();

    let start_time = Instant::now();
    let t1 = compute_rain_collected(&terrain);
    let execution_time = start_time.elapsed().as_micros();

    println!("Execution time trap: {} microseconds", execution_time);
    println!("Water capacity: {}", t1);

    let start_time = Instant::now();
    let t2 = trap_cpp_dp(&terrain);
    let execution_time = start_time.elapsed().as_micros();

    println!("Execution time trap_cpp_dp: {} microseconds", execution_time);
    println!("Water capacity: {}", t2);

    if t1 != t2 {
        println!("Results are different!");
        println!("t1: {}", t1);
        println!("t2: {}", t2);
        println!("Terrain: ");
        println!("{:?}", terrain);
    }
}

