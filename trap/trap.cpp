#include <vector>
#include <iterator>
#include <span>
#include <algorithm>
#include <numeric>
#include <limits>

using namespace std;

extern "C" {
    unsigned long trap_cpp_ffi(long* height, size_t heightSize) {
        std::span<long> v(height, heightSize);
        if (v.size() < 3)
        {
            return 0;
        }

        vector u(v.size(), 0);

        auto it = max_element(begin(v), end(v));

        inclusive_scan(begin(v), next(it), begin(u),
                       [](auto a, auto b)
        {
            return max(a, b);
        });

        inclusive_scan(rbegin(v), reverse_iterator(it),
                       rbegin(u),
                       [](auto a, auto b)
        {
            return max(a, b);
        });

        return transform_reduce(cbegin(u), cend(u), cbegin(v), 0,
                                std::plus<>(),
                                std::minus<>());
    }
}


extern "C" {
    unsigned long trap_cpp_dp_ffi(long* height_a, size_t heightSize) {
        std::span<long> height(height_a, heightSize);
        size_t l = 0, r = height.size()-1;
        long level = std::numeric_limits<long>::min();
        unsigned long  water = 0;
        while (l < r) {
            level = max(level, min(height[l], height[r]));
            if (height[l] < height[r]) {
                water += level - height[l];
                l += 1;
            } else {
                water += level - height[r];
                r -= 1;
            }
        }
        return water;
    }
}
