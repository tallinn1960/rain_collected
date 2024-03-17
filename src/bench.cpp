#include <benchmark/benchmark.h>
#include <numeric>
#include <random>
#include <vector>
#include "trap.hpp"

extern "C" {
    long rainCollected(long *p, long n);
}

std::vector<long> random_terrain(int n) {
  std::vector<long> v(n);
  std::iota(v.begin(), v.end(), 0);
  std::random_device rd;
  std::mt19937 g(rd());
  std::shuffle(v.begin(), v.end(), g);
  return v;
}

auto v = random_terrain(10000000);

void BM_trap_cpp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp(v);
}

void BM_trap_cpp_dp(benchmark::State &state) {
    for (auto _ : state)
        trap_cpp_dp(v);
}

void BM_trap_swift(benchmark::State &state) {
    for (auto _ : state)
        rainCollected(v.data(), v.size());
}

BENCHMARK(BM_trap_cpp);
BENCHMARK(BM_trap_cpp_dp);
BENCHMARK(BM_trap_swift);
