#![allow(missing_docs)]

#[link(name = "trap_cpp", kind = "static")]
extern "C" {
    fn trap_cpp_ffi(v: *mut libc::c_long, size: libc::size_t) -> libc::c_ulong;
}

/// trap function from C++, CppCon version
#[allow(unsafe_code)]
pub fn trap_cpp(v: &[i64]) -> u64 {
    unsafe { trap_cpp_ffi(v.as_ptr() as *mut libc::c_long, v.len()) }
}

#[link(name = "trap_cpp", kind = "static")]
extern "C" {
    fn trap_cpp_dp_ffi(
        v: *mut libc::c_long,
        size: libc::size_t,
    ) -> libc::c_ulong;
}

/// trap function from C++, dp version from leetcode
#[allow(unsafe_code)]
pub fn trap_cpp_dp(v: &[i64]) -> u64 {
    unsafe { trap_cpp_dp_ffi(v.as_ptr() as *mut libc::c_long, v.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(trap_cpp(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_cpp(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_cpp(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_cpp(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_cpp(&terrain11), 33);
    }
}
