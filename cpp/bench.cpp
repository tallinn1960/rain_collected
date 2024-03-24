#include <benchmark/benchmark.h>
#include <random>
#include <vector>
#include "trap.hpp"

extern "C" {
    uint64_t rainCollected(int64_t *p, uint64_t n);
}

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

void BM_trap_cpp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp(v);
}

void BM_trap_cpp_dp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp_dp(v);
}

void BM_compute_rain_collected(benchmark::State &state) {
    for (auto _ : state)
        compute_rain_collected(v);
}

void BM_trap_swift(benchmark::State &state) {
    for (auto _ : state)
        rainCollected(v1.data(), v1.size());
}

BENCHMARK(BM_trap_cpp);
BENCHMARK(BM_trap_cpp_dp);
BENCHMARK(BM_compute_rain_collected);
BENCHMARK(BM_trap_swift);
