#include <cstdint>
#include <iostream>
#include <vector>
#include <random>
#include <algorithm>
#include <chrono>
#include "trap.hpp"

extern "C" {
    uint64_t rainCollected(int64_t *p, uint64_t n);
}

int main() {
    // generate some random input of size 100000 ranging from 0 to 99999
    std::vector<long> v(100000);
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<long> dis(0, 99999);
    std::generate(v.begin(), v.end(), [&]() {
        return dis(gen);
    });

    std::vector<int64_t> v1 = std::vector<int64_t>(v.begin(), v.end());

    // time the call of trap_cpp_dp with v as argument
    auto start = std::chrono::high_resolution_clock::now();
    auto w = trap_cpp_dp(v);
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> diff = end-start;
    std::cout << "Time: " << diff.count() * 1000000 << " micros\n";
    std::cout << "Water: " << w << "\n";

    auto start2 = std::chrono::high_resolution_clock::now();
    auto w2 = rainCollected(v1.data(), v1.size());
    auto end2 = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> diff2 = end2-start2;
    std::cout << "Time: " << diff2.count() * 1000000 << " micros\n";
    std::cout << "Water: " << w2 << "\n";

    return 0;
}
