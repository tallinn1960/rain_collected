#![allow(missing_docs)]

fn main() {
    let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
    let rain_collected = compute_rain_collected(&terrain);
    println!("Rain collected: {}", rain_collected);
    let rain_collected_parallel = compute_rain_collected_parallel(&terrain);
    println!("Rain collected parallel: {}", rain_collected_parallel);
}

fn compute_rain_collected(terrain: &[i64]) -> u64 {
    let n = terrain.len();
    if n < 3 {
        return 0;
    }

    let index_maximum = terrain
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap() // since we ensured above that terrain isn't empty, this is safe
        .0;

    let (leftside, rightside) = terrain.split_at(index_maximum);

    // water + terrain create a stair up left to the maximum elevation and a stair down right to the maximum elevation
    // the elevation of each step of the stair is the maximum elevation seen so far walking towards the maximum elevation
    // water collected is determined by the difference between the elevation of the current step and the actual elevation

    // we calculate the water collected on the left side of the maximum elevation
    let r1 = leftside
        .iter()
        .fold(
            (
                i64::MIN, // tracks maximum elevation
                0u64,     // tracks water collected
            ),
            |acc, &x| {
                let newmax = x.max(acc.0); // update the maximum elevation seen so far
                (newmax, acc.1 + (newmax - x) as u64) // newmax will always be greater as or equal to x,
                                                      // which makes the cast safe
            },
        )
        .1;

    // this time we calculate the water collected on the right side of the maximum elevation
    // we do this by iterating the terrain applying the same fold operation in reverse order
    let r2 = rightside
        .iter()
        .rfold((i64::MIN, 0u64), |acc, &x| {
            let newmax = x.max(acc.0);
            (newmax, acc.1 + (newmax - x) as u64)
        })
        .1;

    r1 + r2
}

// this function is the same as compute_rain_collected but uses rayon to parallelize the computation
// the parallelization is achieved by using rayon's scope function to spawn two threads that calculate the water collected on the left and right side of the maximum elevation
// the results are then joined and summed
// this is indeed much slower than the sequential version
fn compute_rain_collected_parallel(terrain: &[i64]) -> u64 {
    let n = terrain.len();
    if n < 3 {
        return 0;
    }

    let index_maximum = terrain
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap() // since we ensured above that terrain isn't empty, this is safe
        .0;

    std::thread::scope(|s| {
        let (leftside, rightside) = terrain.split_at(index_maximum);

        // water + terrain create a stair up left to the maximum elevation and a stair down right to the maximum elevation
        // the elevation of each step of the stair is the maximum elevation seen so far walking towards the maximum elevation
        // water collected is determined by the difference between the elevation of the current step and the actual elevation

        // we calculate the water collected on the left side of the maximum elevation
        let left = s.spawn(|| {
            leftside
                .iter()
                .fold(
                    (
                        i64::MIN, // tracks maximum elevation
                        0u64,     // tracks water collected
                    ),
                    |acc, &x| {
                        let newmax = x.max(acc.0); // update the maximum elevation seen so far
                        (newmax, acc.1 + (newmax - x) as u64) // newmax will always be greater as or equal to x,
                                                              // which makes the cast safe
                    },
                )
                .1
        });

        // here we calculate the water collected on the right side of the maximum elevation
        // we do this by iterating the terrain in reverse order applying the same fold operation
        let right = s.spawn(|| {
            rightside
                .iter()
                .rfold(
                    (
                        i64::MIN, // tracks maximum elevation
                        0u64,     // tracks water collected
                    ),
                    |acc, &x| {
                        let newmax = x.max(acc.0); // update the maximum elevation seen so far
                        (newmax, acc.1 + (newmax - x) as u64) // newmax will always be greater as or equal to x,
                                                              // which makes the cast safe
                    },
                )
                .1
        });

        left.join().unwrap() + right.join().unwrap()
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
