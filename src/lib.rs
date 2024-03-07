//! # Rain Trapped in Terrain
//! Given a terrain represented by a sequence of integers, each
//! representing the elevation of a spot, calculate the amount of
//! rain that can be trapped in the terrain.

/// Compute the amount of rain that can be trapped in the terrain.
/// # Arguments
/// * `height` - A slice of integers representing the elevation of the terrain.
/// # Returns
/// The amount of rain that can be trapped in the terrain.
/// # Example
/// ```
/// use rain_collected::compute_rain_collected;
/// let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
/// let water_capacity = compute_rain_collected(&terrain);
/// assert_eq!(water_capacity, 6);
/// ```
/// # Note
/// The solution is based on the observation that the terrain with the water trapped in
/// it forms a stair going up to the maximum elevation of the terrain coming from the
/// left and going down after that maximum to the right. So the goal is to calculate
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
///
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
        .scan((hiter.next(), hiter.next_back()), |state, _| {
            if let (Some(left), Some(right)) = *state {
                if left <= right {
                    *state = (hiter.next(), Some(right));
                    Some(left)
                } else {
                    *state = (Some(left), hiter.next_back());
                    Some(right)
                }
            } else {
                None
            }
        })
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
            |acc, &x| {
                let stepsize = x.max(acc.0);
                (stepsize, acc.1 + (stepsize - x) as u64)
            },
        )
        .1 // we are only interested in the water collected
}

/// Variant of compute_rain_collected consuming a vector of ints as input.
/// # Arguments
/// * `height` - A vector of integers representing the elevation of the terrain.
/// # Returns
/// The amount of rain that can be trapped in the terrain.
/// # Example
/// ```
/// use rain_collected::compute_rain_collected2;
/// let terrain = vec![1, 4, 2, 5, 3, 6, 4, 7];
/// let water_capacity = compute_rain_collected2(terrain);
/// assert_eq!(water_capacity, 6);
/// ```
pub fn compute_rain_collected2(height: Vec<i64>) -> u64 {
    let mut hiter = height.into_iter();

    std::iter::repeat(())
        .scan((hiter.next(), hiter.next_back()), |state, _| {
            if let (Some(left), Some(right)) = *state {
                if left <= right {
                    *state = (hiter.next(), Some(right));
                    Some(left)
                } else {
                    *state = (Some(left), hiter.next_back());
                    Some(right)
                }
            } else {
                None
            }
        })
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
        .1 // we are only interested in the water collected}
}

/// previous solution
pub fn compute_rain_collected3(terrain: &[i64]) -> u64 {
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

/// fastest solution from leetcode
pub fn trap(height: &[i64]) -> u64 {
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

/// fastest solution from leetcode, consuming input
pub fn trap_v(height: Vec<i64>) -> u64 {
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
#[allow(unsafe_code)]
/// fastest solution from leetcode, avoiding bounds checks
pub fn trap_unsafe(height: &[i64]) -> u64 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut pool_height = i64::MIN;
    let mut trapped = 0u64;

    while left < right {
        unsafe {
            let leftv = *height.get_unchecked(left);
            let rightv = *height.get_unchecked(right);
            pool_height = pool_height.max(leftv.min(rightv));
            if leftv <= rightv {
                trapped += 0.max(pool_height - leftv) as u64;
                left += 1;
            } else {
                trapped += 0.max(pool_height - rightv) as u64;
                right -= 1;
            }
        }
    }

    trapped
}

/// C++ solution from leetcode
extern crate libc;

#[link(name = "trap", kind = "dylib")]
extern "C" {
    fn trap_cpp_ffi(v: *mut libc::c_long, size: libc::size_t) -> libc::c_ulong;
}
/// trap function from C++
#[allow(unsafe_code)]
pub fn trap_cpp(v: &[i64]) -> u64 {
    // Test case: Call the trap function from Rust
    let size = v.len();
    let result = unsafe { trap_cpp_ffi(v.as_ptr() as *mut libc::c_long, size) };
    result
}

#[link(name = "trap", kind = "dylib")]
extern "C" {
    fn trap_cpp_dp_ffi(v: *mut libc::c_long, size: libc::size_t) -> libc::c_ulong;
}
/// trap function from C++, dp version
#[allow(unsafe_code)]
pub fn trap_cpp_dp(v: &[i64]) -> u64 {
    // Test case: Call the trap function from Rust
    let size = v.len();
    let result = unsafe { trap_cpp_dp_ffi(v.as_ptr() as *mut libc::c_long, size) };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rain_collected() {
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
        assert_eq!(compute_rain_collected(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(compute_rain_collected(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(compute_rain_collected(&terrain11), 33);
    }

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
        assert_eq!(trap(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap(&terrain11), 33);
    }

    #[test]
    fn test_trap_unsafe() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap_unsafe(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap_unsafe(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap_unsafe(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap_unsafe(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap_unsafe(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_unsafe(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap_unsafe(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_unsafe(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_unsafe(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_unsafe(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_unsafe(&terrain11), 33);
    }

    #[test]
    fn test_trap_cpp() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap_cpp(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap_cpp(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap_cpp(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap_cpp(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap_cpp(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_cpp(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_cpp(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_cpp(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_cpp(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_cpp(&terrain11), 33);
    }

    #[test]
    fn test_compute_rain_collected2() {
        let terrain1 = vec![0, 0, 0, 0, 0];
        assert_eq!(compute_rain_collected2(terrain1), 0);

        let terrain2 = vec![1, 2, 3, 4, 5];
        assert_eq!(compute_rain_collected2(terrain2), 0);

        let terrain3 = vec![5, 4, 3, 2, 1];
        assert_eq!(compute_rain_collected2(terrain3), 0);

        let terrain4 = vec![1, 2, 3, 2, 1];
        assert_eq!(compute_rain_collected2(terrain4), 0);

        let terrain5 = vec![1, 2, 3, 2, 4, 1];
        assert_eq!(compute_rain_collected2(terrain5), 1);

        let terrain6 = vec![1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(compute_rain_collected2(terrain6), 6);

        let terrain7 = vec![2, 1, 2];
        assert_eq!(compute_rain_collected2(terrain7), 1);

        let terrain8 = vec![5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(compute_rain_collected2(terrain8), 5);

        let terrain9 = vec![0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(compute_rain_collected2(terrain9), 7);

        let terrain10 = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(compute_rain_collected2(terrain10), 9);

        let terrain11 = vec![0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(compute_rain_collected2(terrain11), 33);
    }

    #[test]
    fn test_compute_rain_collected3() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(compute_rain_collected3(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(compute_rain_collected3(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(compute_rain_collected3(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(compute_rain_collected3(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(compute_rain_collected3(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(compute_rain_collected3(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(compute_rain_collected3(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(compute_rain_collected3(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(compute_rain_collected3(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(compute_rain_collected3(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(compute_rain_collected3(&terrain11), 33);
    }

    #[test]
    fn test_trap_v() {
        let terrain1 = vec![0, 0, 0, 0, 0];
        assert_eq!(trap_v(terrain1), 0);

        let terrain2 = vec![1, 2, 3, 4, 5];
        assert_eq!(trap_v(terrain2), 0);

        let terrain3 = vec![5, 4, 3, 2, 1];
        assert_eq!(trap_v(terrain3), 0);

        let terrain4 = vec![1, 2, 3, 2, 1];
        assert_eq!(trap_v(terrain4), 0);

        let terrain5 = vec![1, 2, 3, 2, 4, 1];
        assert_eq!(trap_v(terrain5), 1);

        let terrain6 = vec![1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_v(terrain6), 6);

        let terrain7 = vec![2, 1, 2];
        assert_eq!(trap_v(terrain7), 1);

        let terrain8 = vec![5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_v(terrain8), 5);

        let terrain9 = vec![0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_v(terrain9), 7);

        let terrain10 = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap_v(terrain10), 9);

        let terrain11 = vec![0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_v(terrain11), 33);
    }

    #[test]
    fn test_trap_cpp_dp() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap_cpp_dp(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap_cpp_dp(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap_cpp_dp(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap_cpp_dp(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap_cpp_dp(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_cpp_dp(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap_cpp_dp(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_cpp_dp(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_cpp_dp(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_cpp_dp(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_cpp_dp(&terrain11), 33);
    }
}
