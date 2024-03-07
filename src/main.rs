//! # Rain trapped in terrain
//! Given a terrain represented by a sequence of integers, each
//! representing the elevation of a spot, calculate the amount of
//! rain that can be trapped in the terrain.

use rain_collected::{compute_rain_collected, trap};
const N: i64 = 100000;

fn main() {
    use std::time::Instant;

    let terrain: Vec<i64> = (0..N).map(|_| (rand::random::<i64>() % N)).collect();

    let start_time = Instant::now();
    let t1 = compute_rain_collected(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!("Execution time trap: {} microseconds", execution_time);
    println!("Water capacity: {}", t1);

    let start_time = Instant::now();
    let t2 = trap(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!("Execution time trap2: {} microseconds", execution_time);
    println!("Water capacity: {}", t2);

    if t1 != t2 {
        println!("Results are different!");
        println!("t1: {}", t1);
        println!("t2: {}", t2);
        println!("Terrain: ");
        println!("{:?}", terrain);
    }
}

