#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

// a * x + b * y = gcd(a, b)
int64_t extgcd(int64_t a, int64_t b, int64_t& x, int64_t& y) {
    if (b == 0) {
        x = 1;
        y = 0;
        return a;
    }

    int64_t g = extgcd(b, a % b, y, x);
    y -= a / b * x;
    return g;
}

int64_t test(int64_t a, int64_t b) {
    // k = -ya
    // k + 1 = -zb
    // => 1 = ya + zb

    int64_t y, z;
    auto g = extgcd(a, b, y, z);

    if (g != 1) {
        return 1L << 60;
    }

    if (y < 0) {
        return -y * a % (a * b);
    } else {
        return -(y * a % (a * b)) + a * b;
    }
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