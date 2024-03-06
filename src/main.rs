//! rain_collected
//! 
//! This program calculates the amount of water that can be collected in a terrain.
//! The terrain is represented as a vector of elevations. The elevation at each index of the vector
//! represents the height of the terrain at that point.
//!
use rain_collected::{compute_rain_collected, compute_rain_collected_parallel};

const N: usize = 10000;

fn main() {
    use std::time::Instant;
    // generate a large terrain with N randome elevations between 0 and 100
    let terrain: Vec<i64> = (0..N).map(|_| rand::random::<i64>() % 100).collect();

    let start_time = Instant::now();
    compute_rain_collected(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!(
        "Execution time compute_rain_collected: {} microseconds",
        execution_time
    );

    let start_time = Instant::now();
    compute_rain_collected_parallel(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!(
        "Execution time compute_rain_collected_parallel: {} microseconds",
        execution_time
    );
}
