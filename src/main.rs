//! rain_collected
fn main() {
    let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
    let rain_collected = compute_rain_collected(&terrain);
    println!("Rain collected: {}", rain_collected);
    let rain_collected_parallel = compute_rain_collected_parallel(&terrain);
    println!("Rain collected parallel: {}", rain_collected_parallel);
}

/// Compute the capacity of water that can be collected in the given terrain.
fn compute_rain_collected(terrain: &[i64]) -> u64 {
    let n = terrain.len();
    if n < 3 {
        return 0;
    }

    // Water + terrain create a stair up left to the maximum elevation and a stair down right to the maximum elevation.
    // The elevation of each step of the stair is the maximum elevation seen so far walking towards the maximum elevation from either side.
    // Water collected is determined by the difference between the elevation of the current step and the actual elevation.

    let index_maximum = terrain
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap() // since we ensured above that terrain isn't empty, this is safe
        .0;

    let (terrain_left_of_max_elevation, terrain_right_of_max_elevation) =
        terrain.split_at(index_maximum);

    // We calculate the water collected on the left side of the maximum elevation.
    let water_capacity_left = terrain_left_of_max_elevation
        .iter()
        .fold(
            (
                i64::MIN, // .0: tracks maximum elevation
                0u64,     // .1: tracks water collected
            ),
            |acc, &x| {
                let stepsize = x.max(acc.0); // update the maximum elevation seen so far
                (stepsize, acc.1 + (stepsize - x) as u64) // stepsize will always be greater as or equal to x,
                                                          // which makes the cast safe
            },
        )
        .1; // we are only interested in the water collected

    // This time we calculate the water collected on the right side of the maximum elevation.
    // We do this by iterating the terrain right to the max elevation applying the same fold operation in reverse order.
    let water_capacity_right = terrain_right_of_max_elevation
        .iter()
        .rfold((i64::MIN, 0u64), |acc, &x| {
            let stepsize = x.max(acc.0);
            (stepsize, acc.1 + (stepsize - x) as u64)
        })
        .1;

    water_capacity_left + water_capacity_right
}

/// This function is the same as compute_rain_collected but uses threads to parallelize the computation.
/// This is suprisingly much slower than the sequential version.
fn compute_rain_collected_parallel(terrain: &[i64]) -> u64 {
    let n = terrain.len();
    if n < 3 {
        return 0;
    }

    let index_maximum = terrain
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0;

    let (terrain_left_of_max_elevation, terrain_right_of_max_elevation) =
        terrain.split_at(index_maximum);

    std::thread::scope(|s| {
        let water_capacity_left = s.spawn(|| {
            terrain_left_of_max_elevation
                .iter()
                .fold((i64::MIN, 0u64), |acc, &x| {
                    let stepsize = x.max(acc.0);
                    (stepsize, acc.1 + (stepsize - x) as u64)
                })
                .1
        });

        let water_capacity_right = s.spawn(|| {
            terrain_right_of_max_elevation
                .iter()
                .rfold((i64::MIN, 0u64), |acc, &x| {
                    let stepsize = x.max(acc.0);
                    (stepsize, acc.1 + (stepsize - x) as u64)
                })
                .1
        });

        water_capacity_left.join().unwrap() + water_capacity_right.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rain_collected() {
        // Test case 1: Terrain with no elevation difference
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(compute_rain_collected(&terrain1), 0);

        // Test case 2: Terrain with increasing elevation
        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(compute_rain_collected(&terrain2), 0);

        // Test case 3: Terrain with decreasing elevation
        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(compute_rain_collected(&terrain3), 0);

        // Test case 4: Terrain with single peak
        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(compute_rain_collected(&terrain4), 0);

        // Test case 5: Terrain with multiple peaks
        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(compute_rain_collected(&terrain5), 1);

        // Test case 6: Terrain with uneven peaks
        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(compute_rain_collected(&terrain6), 6);

        // Test case 7: Terrain with only three elevations
        let terrain7 = [2, 1, 2];
        assert_eq!(compute_rain_collected(&terrain7), 1);

        // Test case 8: Terrain with a plateau at maximum elevation
        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(compute_rain_collected(&terrain8), 5);
    }

    const N: usize = 10000;

    #[test]
    fn calculate_execution_time() {
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
}
