#include <bits/stdc++.h>

#include <atcoder/math>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;
using namespace atcoder;

int64_t test(int64_t a, int64_t b) {
    auto [y, z] = crt({0L, -1L}, {a, b});

    if (y == 0 && z == 0) {
        return 1L << 60;
    }

    if (y == 0) {
        return z;
    }

    return y;
}

int main() {
    int64_t n;
    cin >> n;

    auto m = 2 * n;

    int64_t ans = 2 * n - 1;
    for (int64_t x = 2; x * x <= m; ++x) {
        if (m % x > 0) {
            continue;
        }

        ans = min({ans, test(x, m / x), test(m / x, x)});
    }

    cout << ans << endl;
}