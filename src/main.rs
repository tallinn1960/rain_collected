#![allow(missing_docs)]

const N: i64 = 10000;

fn main() {
    use std::time::Instant;

    let terrain: Vec<i64> = (0..N).map(|_| rand::random::<i64>() % N).collect();

    let start_time = Instant::now();
    let t = trap(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!("Execution time trap: {} microseconds", execution_time);
    println!("Water capacity: {}", t);
}

fn trap(terrain: &[i64]) -> u64 {
   
    if terrain.len() < 3 {
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

    let water_capacity_left = terrain_left_of_max_elevation
        .iter()
        .fold((i64::MIN, 0u64), |acc, &x| {
            let stepsize = x.max(acc.0);
            (stepsize, acc.1 + (stepsize - x) as u64)
        })
        .1;

    let water_capacity_right = terrain_right_of_max_elevation
        .iter()
        .rfold((i64::MIN, 0u64), |acc, &x| {
            let stepsize = x.max(acc.0);
            (stepsize, acc.1 + (stepsize - x) as u64)
        })
        .1;

    water_capacity_left + water_capacity_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap(&terrain8), 5);
    }
}
