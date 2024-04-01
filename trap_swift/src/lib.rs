#![allow(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(unsafe_code)]
pub fn trap_swift(terrain: &[i64]) -> u64 {
    unsafe { rainCollected(terrain.as_ptr(), terrain.len() as u64) }
}
