#include "trap.hpp"
#include "trap_swift.h"
#include <benchmark/benchmark.h>
#include <random>
#include <vector>

std::vector<long> random_terrain(int n) {
    // generate some random input ranging from 0 to 99999
    std::vector<long> v(n);
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<long> dis(0, 99999);
    std::generate(v.begin(), v.end(), [&]() {
        return dis(gen);
    });
    return v;
}

auto v = random_terrain(10000000);
auto v1 = std::vector<int64_t>(v.begin(), v.end());

extern "C" {

/// C interface to compute_rain_collected
/// # Safety
/// The caller must ensure that the pointer is valid and points to a valid
/// slice of integers.
/// # Arguments
/// * `height` - A pointer to a slice of integers representing the elevation of the terrain.
/// * `len` - The length of the slice.
/// # Returns
/// The amount of rain that can be trapped in the terrain.
/// # Example
/// ```
/// use trap_rust::compute_rain_collected_ffi;
/// let terrain = [1, 4, 2, 5, 3, 6, 4, 7];
/// let water_capacity = unsafe { compute_rain_collected_ffi(terrain.as_ptr(), terrain.len()) };
/// assert_eq!(water_capacity, 6);
/// ```
/// # Note
/// This function is a wrapper around the safe function compute_rain_collected.
/// It is meant to be called from C code.
/// ```c
/// #include <stdint.h>
/// uint64_t compute_rain_collected(int64_t *height, size_t len);
/// ```
    uint64_t compute_rain_collected_ffi(const int64_t *height, uintptr_t len);

}

void BM_trap_cpp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp(v);
}

void BM_trap_cpp_dp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp_dp(v);
}

void BM_trap_swift(benchmark::State &state) {
    for (auto _ : state) {
        auto __ = trap_swift::rainCollected(v1.data(), v1.size());
    };
}

void BM_trap_rust(benchmark::State &state) {
    for (auto _ : state) {
        auto __ = compute_rain_collected_ffi(v1.data(), v1.size());
    };
}

BENCHMARK(BM_trap_cpp);
BENCHMARK(BM_trap_cpp_dp);
BENCHMARK(BM_trap_swift);
BENCHMARK(BM_trap_rust);

BENCHMARK_MAIN();