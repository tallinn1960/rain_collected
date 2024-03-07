#include <iterator>
#include <span>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

unsigned long trap(std::span<long> v)
{
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

extern "C" {
    unsigned long trap_cpp_ffi(long* height, int heightSize) {
        std::span<long> v(height, heightSize);
        return trap(v);
    }
}

