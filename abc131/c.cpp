#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t gcd(int64_t x, int64_t y) {
    if (x == 0) {
        return y;
    }

    return gcd(y % x, x);
}

int64_t lcm(int64_t x, int64_t y) {
    auto g = gcd(x, y);

    return x * y / g;
}

main() {
    int64_t a, b, c, d;
    cin >> a >> b >> c >> d;

    auto l = lcm(c, d);

    auto n_c = b / c - (a - 1) / c;
    auto n_d = b / d - (a - 1) / d;
    auto n_l = b / l - (a - 1) / l;

    auto ans = b - a + 1 - n_c - n_d + n_l;

    cout << ans << endl;
}