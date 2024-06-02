#[link(name = "trap_zig", kind = "static")]
extern "C" {
    fn trap_zig(v: *mut libc::c_long, size: libc::size_t) -> libc::c_ulong;
}

pub fn trap_zig_ffi(terrain: &[i64]) -> u64 {
    unsafe { trap_zig(terrain.as_ptr() as *mut libc::c_long, terrain.len())}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_zig() {
        let terrain1 = [0, 0, 0, 0, 0];
        assert_eq!(trap_zig_ffi(&terrain1), 0);

        let terrain2 = [1, 2, 3, 4, 5];
        assert_eq!(trap_zig_ffi(&terrain2), 0);

        let terrain3 = [5, 4, 3, 2, 1];
        assert_eq!(trap_zig_ffi(&terrain3), 0);

        let terrain4 = [1, 2, 3, 2, 1];
        assert_eq!(trap_zig_ffi(&terrain4), 0);

        let terrain5 = [1, 2, 3, 2, 4, 1];
        assert_eq!(trap_zig_ffi(&terrain5), 1);

        let terrain6 = [1, 4, 2, 5, 3, 6, 4, 7];
        assert_eq!(trap_zig_ffi(&terrain6), 6);

        let terrain7 = [2, 1, 2];
        assert_eq!(trap_zig_ffi(&terrain7), 1);

        let terrain8 = [5, 4, 2, 6, 6, 6, 4, 5];
        assert_eq!(trap_zig_ffi(&terrain8), 5);

        let terrain9 = [0, 1, -1, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_zig_ffi(&terrain9), 7);

        let terrain10 = [4, 2, 0, 3, 2, 5];
        assert_eq!(trap_zig_ffi(&terrain10), 9);

        let terrain11 = [0, -6, 0, -2, 8, -9, 0, 8, 9, -5];
        assert_eq!(trap_zig_ffi(&terrain11), 33);
    }
}