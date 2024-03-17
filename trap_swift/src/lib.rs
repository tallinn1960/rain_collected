#![allow(missing_docs)]

#[link(name = "trap_swift", kind = "static")]
extern "C" {
    fn rainCollected(v: *mut libc::c_long, size: libc::size_t)
        -> libc::c_ulong;
}

#[allow(unsafe_code)]
/// trap function from Swift
pub fn trap_swift(v: &[i64]) -> u64 {
    unsafe { rainCollected(v.as_ptr() as *mut libc::c_long, v.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trap_swift() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap_swift(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap_swift(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap_swift(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap_swift(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap_swift(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_swift(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap_swift(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_swift(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_swift(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_swift(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_swift(&terrain11), 33);
    }
}