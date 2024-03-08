#include <iostream>
#include <vector>
#include <random>
#include <algorithm>
#include "trap.hpp"

int main() {
    // generate some random input of size 100000
    std::vector<long> v(100000);
    std::iota(v.begin(), v.end(), 0);
    std::random_device rd;
    std::mt19937 g(rd());
    std::shuffle(v.begin(), v.end(), g);

    // time the call of trap_cpp_dp with v as argument
    auto start = std::chrono::high_resolution_clock::now();
    auto w = trap_cpp_dp(v);
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> diff = end-start;
    std::cout << "Time: " << diff.count() * 1000000 << " micros\n";
    std::cout << "Water: " << w << "\n";
    return 0;
}
