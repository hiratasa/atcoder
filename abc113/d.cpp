#include <iostream>
#include <vector>
#include <utility>
#include <string>
#include <numeric>
#include <cmath>
#include <cassert>
#include <iomanip>
#include <sstream>
#include <algorithm>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <queue>

using namespace std;

constexpr auto M = 1000000007;

// x は高々9
int64_t fib(int x) {
    switch (x) {
        case -1:
            return 0;
        case 0:
        case 1:
            return 1;
        case 2:
            return 2;
        case 3:
            return 3;
        case 4:
            return 5;
        case 5:
            return 8;
        case 6:
            return 13;
        case 7:
            return 21;
        case 8:
            return 34;
        default:
            assert(false);
            return -1;
    }
}

int64_t solve(int h, int w, int k) {
    vector<int64_t> current(w + 2, 0);
    vector<int64_t> next(w + 2, 0);

    current[1] = 1;

    for (int hh = 1; hh <= h; ++hh) {
        for (int x = 1; x <= w; ++x) {
            next[x] = (current[x - 1] * fib(x - 2) * fib(w - x) + current[x] * fib(x - 1) * fib(w - x) + current[x + 1] * fib(x - 1) * fib(w - x - 1)) % M;
        }
        current.swap(next);
    }

    return current[k];
} 

int main() {
    int h, w, k;
    cin >> h >> w >> k;

    cout << solve(h, w, k) << endl;

    return 0;
}