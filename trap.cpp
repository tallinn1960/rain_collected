#include <chrono>
#include <iostream>
#include <cassert>
#include <iterator>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

int trap(vector<int> &v)
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

int main()
{
    // Test case 1: Empty vector
    {
        vector<int> v;
        int result = trap(v);
        assert(result == 0);
        cout << "Test case 1 passed." << endl;
    }

    // Test case 2: No bars to trap water
    {
        vector<int> v = {1, 2, 3, 4, 5};
        int result = trap(v);
        assert(result == 0);
        cout << "Test case 2 passed." << endl;
    }

    // Test case 3: All bars have the same height
    {
        vector<int> v = {3, 3, 3, 3, 3};
        int result = trap(v);
        assert(result == 0);
        cout << "Test case 3 passed." << endl;
    }

    // Test case 4: Bars form a "U" shape
    {
        vector<int> v = {3, 0, 1, 0, 3};
        int result = trap(v);
        assert(result == 8);
        cout << "Test case 4 passed." << endl;
    }

    // Test case 5: Bars form a "W" shape
    {
        vector<int> v = {1, 0, 2, 0, 1};
        int result = trap(v);
        assert(result == 2);
        cout << "Test case 5 passed." << endl;
    }

    // Test case 6: Bars form a staircase shape
    {
        vector<int> v = {1, 2, 3, 4, 5};
        int result = trap(v);
        assert(result == 0);
        cout << "Test case 6 passed." << endl;
    }

    // Test case 7: Randomly filled vector, time measurement
    {
        int N = 100000; // Set the size of the vector
        vector<int> v(N);

        // Fill the vector with random integers modulo N
        for (int i = 0; i < N; i++) {
            v[i] = rand() % N;
        }

        // Start the timer
        auto start = std::chrono::high_resolution_clock::now();

        // Call the trap function
        int result = trap(v);

        // Stop the timer
        auto end = std::chrono::high_resolution_clock::now();

        // Calculate the duration
        auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();

        // Print the result and duration
        cout << "Time taken: " << duration << " microseconds" << endl;
    }
    cout << "All test cases passed." << endl;

    return 0;
}