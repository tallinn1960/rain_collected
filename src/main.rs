#![allow(missing_docs)]

const N: i64 = 100000;

fn main() {
    use std::time::Instant;

    let terrain: Vec<i64> = (0..N).map(|_| (rand::random::<i64>() % N)).collect();

    let start_time = Instant::now();
    let t1 = trap(&terrain);
    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time).as_micros();

    println!("Execution time trap: {} microseconds", execution_time);
    println!("Water capacity: {}", t1);

    let start_time = Instant::now();
    let t2 = trap2(&terrain);
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

fn trap(height: &[i64]) -> u64 {
    let mut hiter = height.into_iter();
    std::iter::repeat(())
        .scan(
            (hiter.next().copied(), hiter.next_back().copied()),
            |state, _| {
                if let (Some(left), Some(right)) = *state {
                    if left <= right {
                        *state = (hiter.next().copied(), Some(right));
                        Some(left)
                    } else {
                        *state = (Some(left), hiter.next_back().copied());
                        Some(right)
                    }
                } else {
                    None
                }
            },
        )
        .fold((i64::MIN, 0u64), |acc, x| {
            let stepsize = x.max(acc.0);
            (stepsize, acc.1 + (stepsize - x) as u64)
        })
        .1
}

// fastest solution from leetcode
fn trap2(height: &[i64]) -> u64 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut pool_height = i64::MIN;
    let mut trapped = 0u64;

    while left < right {
        pool_height = pool_height.max(height[left].min(height[right]));
        if height[left] <= height[right] {
            trapped += 0.max(pool_height - height[left]) as u64;
            left += 1;
        } else {
            trapped += 0.max(pool_height - height[right]) as u64;
            right -= 1;
        }
    }

    trapped
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

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap2(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap2(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap2(&terrain11), 33);
    }

    #[test]
    fn test_trap2() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap2(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap2(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap2(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap2(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap2(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap2(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap2(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap2(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap2(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap2(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap2(&terrain11), 33);
    }
}
