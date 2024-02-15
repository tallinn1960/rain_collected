#![allow(missing_docs)]

fn main() {
    let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
    let rain_collected = compute_rain_collected(&terrain);
    println!("Rain collected: {}", rain_collected);
}

fn compute_rain_collected(terrain: &[i32]) -> i32 {
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

    let r1 = terrain[..index_maximum]
        .iter()
        .scan(0, |acc, &x| {
            *acc = x.max(*acc);
            Some(*acc)
        })
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (x - terrain[i]));

    let r2 = terrain[index_maximum..]
        .iter()
        .rev()
        .scan(0, |acc, &x| {
            *acc = x.max(*acc);
            Some(*acc)
        })
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (x - terrain[n - 1 - i]));

    r1 + r2
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

    #[test]
    fn calculate_execution_time() {
        use std::time::Instant;
        // generate a large terrain with 1000 randome elevations between 0 and 100
        let terrain: Vec<i32> = (0..10000).map(|_| rand::random::<i32>() % 100).collect();

        let start_time = Instant::now();
        compute_rain_collected(&terrain);
        let end_time = Instant::now();
        let execution_time = end_time.duration_since(start_time).as_micros();

        println!("Execution time: {} microseconds", execution_time);
    }
}
