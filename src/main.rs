//! # Rain trapped in terrain
//! Given a terrain represented by a sequence of integers, each
//! representing the elevation of a spot, calculate the amount of
//! rain that can be trapped in the terrain.
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

/// Compute the amount of rain that can be trapped in the terrain.
/// # Arguments
/// * `height` - A slice of integers representing the elevation of the terrain.
/// # Returns
/// The amount of rain that can be trapped in the terrain.
/// # Example
/// ```
/// let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
/// let water_capacity = compute_rain_collected(&terrain);
/// assert_eq!(water_capacity, 6);
/// ```
/// # Note
/// The solution is based on the observation that the terrain with the water trapped in
/// it forms a stair going up to the maximum elevation of the terrain coming from the l
/// eft and going down after that maximum to the right. So the goal is to calculate
/// the size of the step corresponding to a given spot of the terrain left and right
/// of the maximum elevation. The water collected on that spot is the difference
/// between this stepsize and the actual elevation of the spot. If the maximum happens
/// to be on the last spot of the terrain, the solution would be a simple fold operation,
/// tracking the maxmium elevation seen while iterating over the spots from left to right,
/// and summing over the water collected on each spot.
///
/// With the maximum elevation anywhere in the terrain, we could compute the solution by applying the fold
/// operation to all spots left of the maximum in order and to all spots right of the maximum in reverse
/// order. This would require us to find the position of the maximum elevation, split the terrain at that
/// position and apply the fold operations to either side. That makes two full iterations over the
/// terrain, one for finding the maximum position, the other for the two folds.
pub fn compute_rain_collected(height: &[i64]) -> u64 {
    let mut hiter = height.into_iter();

    std::iter::repeat(())
        // But we can also reorder the spots by looking at each
        // spot left and right of the maximum using the
        // DoubleEndedIterator trait methods next() and
        // next_back(). We take the value given by each method,
        // determine which one is the minimum, take that as the
        // next value of the new sequence and advance just the
        // corresponding end. scan will keep track of the
        // values at both ends and emits the new sequence, ending
        // it when both ends meet. This emits all spots just in a
        // new order.
        //
        // We use the DoubleEndedIterator trait methods to avoid
        // indexing the terrain, which would result in unnecessary
        // range checks on the indices.  There would never be a
        // range check panic using indices,  but the checks and
        // branches are there and cost CPU time.
        //
        // This solution gives the most efficient assembly code
        // and performs best.
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
        // The newly ordered sequence traps the same amount of
        // water as the former one (yes, you got it, proof is left
        // to the reader), but has the maximum as the
        // last spot. We can then apply the fold operation
        // described above for the "maximum is last" case. In
        // an iterator pipeline this will iterate over the
        // terrain only once.
        .fold(
            (
                i64::MIN, // keeps track of the stepsize of the stair
                0u64,     // keeps track of the water collected
            ),
            |acc, x| {
                let stepsize = x.max(acc.0);
                (stepsize, acc.1 + (stepsize - x) as u64)
            },
        )
        .1 // we are only interested in the water collected
}

// fastest solution from leetcode
fn trap(height: &[i64]) -> u64 {
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
        assert_eq!(compute_rain_collected(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(compute_rain_collected(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(compute_rain_collected(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(compute_rain_collected(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(compute_rain_collected(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(compute_rain_collected(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(compute_rain_collected(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(compute_rain_collected(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap(&terrain11), 33);
    }

    #[test]
    fn test_trap2() {
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
        assert_eq!(trap(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap(&terrain11), 33);
    }
}
